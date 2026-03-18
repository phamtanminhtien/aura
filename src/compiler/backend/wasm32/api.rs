//! wasm-bindgen entry points for the Aura compiler.
//!
//! This module exposes `compile()` and `emit_ir()` to JavaScript.
//! Output from `print()` statements is captured via a thread-local buffer
//! instead of going to stdout (which doesn't exist in wasm32).

use wasm_bindgen::prelude::*;

use crate::compiler::frontend::lexer::Lexer;
use crate::compiler::frontend::parser::Parser;
use crate::compiler::interp::{Interpreter, Value};
use crate::compiler::intrinsic::register_analyzer_intrinsics;
use crate::compiler::ir::lower::Lowerer;
use crate::compiler::ir::opt::Optimizer;
use crate::compiler::sema::checker::SemanticAnalyzer;
use std::cell::RefCell;

// ── Thread-local output capture ─────────────────────────────────────────────

thread_local! {
    static OUTPUT_BUF: RefCell<String> = RefCell::new(String::new());
}

fn capture_output(line: &str) {
    OUTPUT_BUF.with(|buf| {
        buf.borrow_mut().push_str(line);
        buf.borrow_mut().push('\n');
    });
}

fn take_output() -> String {
    OUTPUT_BUF.with(|buf| {
        let s = buf.borrow().clone();
        buf.borrow_mut().clear();
        s
    })
}

// ── Public JS-facing types ───────────────────────────────────────────────────

/// Result returned from a `compile()` call.
#[wasm_bindgen]
pub struct CompileResult {
    ok: bool,
    output: String,
    errors: String,
}

#[wasm_bindgen]
impl CompileResult {
    /// Whether the compilation and execution succeeded without errors.
    pub fn ok(&self) -> bool {
        self.ok
    }

    /// Captured stdout (from `print()` / `println()` calls in the Aura program).
    pub fn output(&self) -> String {
        self.output.clone()
    }

    /// Compiler/interpreter diagnostics, empty if `ok()` is true.
    pub fn errors(&self) -> String {
        self.errors.clone()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn lex_parse(source: &str) -> Result<(Lexer<'_>, crate::compiler::ast::Program), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens, "<playground>".to_string());
    let program = parser.parse_program();

    let mut errors = String::new();
    if lexer.diagnostics.has_errors() {
        for d in lexer.diagnostics.errors() {
            errors.push_str(&format!("{}:{}: {}\n", d.line, d.column, d.message));
        }
    }
    if parser.diagnostics.has_errors() {
        for d in parser.diagnostics.errors() {
            errors.push_str(&format!("{}:{}: {}\n", d.line, d.column, d.message));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok((lexer, program))
}

fn sema_check(program: crate::compiler::ast::Program) -> Result<SemanticAnalyzer, String> {
    let mut analyzer = SemanticAnalyzer::new();
    register_analyzer_intrinsics(&mut analyzer);
    // No stdlib in wasm (no filesystem). Intrinsics only.
    analyzer.analyze(program);
    if analyzer.diagnostics.has_errors() {
        let msg = analyzer
            .diagnostics
            .errors()
            .iter()
            .map(|d| format!("{}:{}: {}", d.line, d.column, d.message))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(msg);
    }
    Ok(analyzer)
}

fn build_interpreter() -> Interpreter {
    let mut interp = Interpreter::new();

    // Override `print` / `println` to write into the thread-local buffer
    // instead of stdout.
    interp.env.insert(
        "print".to_string(),
        Value::NativeFunction(std::rc::Rc::new(|args: Vec<Value>| {
            let text = args
                .into_iter()
                .map(|v| format_value(&v))
                .collect::<Vec<_>>()
                .join(" ");
            capture_output(&text);
            Value::Void
        })),
    );

    interp.env.insert(
        "println".to_string(),
        Value::NativeFunction(std::rc::Rc::new(|args: Vec<Value>| {
            let text = args
                .into_iter()
                .map(|v| format_value(&v))
                .collect::<Vec<_>>()
                .join(" ");
            capture_output(&text);
            Value::Void
        })),
    );

    interp.print_handler = Some(std::rc::Rc::new(|s| {
        capture_output(s);
    }));

    interp
}

fn format_value(val: &Value) -> String {
    match val {
        Value::Int(n) => n.to_string(),
        Value::Int64(n) => n.to_string(),
        Value::Float(f) => {
            if *f == f.floor() && f.abs() < 1e15 {
                format!("{}", *f as i64)
            } else {
                format!("{}", f)
            }
        }
        Value::Boolean(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Void => String::new(),
        other => format!("{:?}", other),
    }
}

// ── Exported API ─────────────────────────────────────────────────────────────

/// Compile and run an Aura source string, returning captured output.
///
/// Pipeline: Lex → Parse → SemanticAnalysis → Interpret
/// `print()` / `println()` calls are captured to `CompileResult::output()`.
#[wasm_bindgen]
pub fn compile(source: &str) -> CompileResult {
    // Clear any leftover output from a previous run.
    let _ = take_output();

    // Lex + Parse
    let program = match lex_parse(source) {
        Ok((_, p)) => p,
        Err(errors) => {
            return CompileResult {
                ok: false,
                output: String::new(),
                errors,
            }
        }
    };

    // Semantic analysis
    let _analyzer = match sema_check(program.clone()) {
        Ok(a) => a,
        Err(errors) => {
            return CompileResult {
                ok: false,
                output: String::new(),
                errors,
            }
        }
    };

    // Interpret
    let mut interp = build_interpreter();
    // Register remaining intrinsics (string, array, etc.) that are safe in wasm.
    // We skip fs, net, date as they require OS calls.
    crate::compiler::intrinsic::string::register_string_intrinsics(&mut |name, val| {
        interp.env.insert(name, val);
    });
    crate::compiler::intrinsic::array::register_array_intrinsics(&mut |name, val| {
        interp.env.insert(name, val);
    });

    interp.interpret(program);

    let output = take_output();
    let errors = String::new();

    CompileResult {
        ok: true,
        output,
        errors,
    }
}

/// Parse and lower an Aura source string to IR, returning the pretty-printed IR.
///
/// Useful for the "Show IR" feature of the playground.
#[wasm_bindgen]
pub fn emit_ir(source: &str) -> String {
    let program = match lex_parse(source) {
        Ok((_, p)) => p,
        Err(e) => return format!("// Parse error:\n// {}", e),
    };

    if let Err(e) = sema_check(program.clone()) {
        return format!("// Semantic error:\n// {}", e);
    }

    let mut lowerer = Lowerer::new();
    let module = lowerer.lower_program(program);
    let mut opt = Optimizer::new();
    let module = opt.optimize(module);
    format!("{}", module)
}

/// Set a panic hook so panics surface as JS errors in the browser console.
#[wasm_bindgen(start)]
pub fn init() {
    // console_error_panic_hook is an optional dev-time dep.
    // Enable it by adding `console_error_panic_hook = "0.1"` to [dependencies]
    // and adding `features = ["console_error_panic_hook"]` to the wasm feature.
}
