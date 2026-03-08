# Plan 3 Phase 3: Real-time Feedback

## 📌 Goals

Connect the `SemanticAnalyzer` to the LSP to provide instant feedback for type errors, name resolution issues, and other semantic violations.

## 📝 Tasks

- [ ] Update `SemanticAnalyzer` to collect multiple diagnostics instead of returning a single `Err`
- [ ] Integrate `SemanticAnalyzer` into `src/lsp/server.rs`'s `on_change` loop
- [ ] Map semantic errors to LSP `Diagnostic` objects
- [ ] Implement `textDocument/hover` to show type information from the symbol table
- [ ] Verify real-time feedback with type mismatch examples
