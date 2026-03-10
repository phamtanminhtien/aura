use crate::compiler::interp::value::Value;
use std::rc::Rc;

pub fn register_timer_intrinsics(interp_env: &mut dyn FnMut(String, Value)) {
    // These will be handled by the interpreter's event loop logic
    // We just need to provide a way for the interpreter to receive these calls

    // __timer_set_timeout(callback: function, delay: i32) -> i32
    interp_env(
        "__timer_set_timeout".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            // This is a placeholder; the actual logic is in the interpreter
            // but we need the native function to exist so it can be called.
            // The interpreter will intercept calls to these functions.
            if args.len() < 2 {
                return Value::Int(0);
            }
            Value::Int(0)
        })),
    );

    // __timer_set_interval(callback: function, delay: i32) -> i32
    interp_env(
        "__timer_set_interval".to_string(),
        Value::NativeFunction(Rc::new(|args| {
            if args.len() < 2 {
                return Value::Int(0);
            }
            Value::Int(0)
        })),
    );

    // __timer_clear(id: i32) -> void
    interp_env(
        "__timer_clear".to_string(),
        Value::NativeFunction(Rc::new(|_args| Value::Void)),
    );
}
