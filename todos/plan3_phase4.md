# Plan 3 Phase 4: Self-Hosted Stdlib Docs

## 📌 Goals

Write the core library in Aura itself and upgrade the LSP to support autocomplete, documentation generation, and cross-file navigation.

## 📝 Tasks

- [x] Initialize `stdlib/` directory and create `io.aura` and `math.aura`
- [x] Add support for "Doc Comments" (e.g., `///`) in the Lexer and AST
- [x] Implement `textDocument/completion` for basic identifier and member access
- [x] Implement `textDocument/documentSymbol` to provide an outline of the current file
- [x] Update `SemanticAnalyzer` to load and analyze `stdlib` files automatically
- [x] Verify with `stdlib_test.aura` — semantic analysis correctly resolves `Math.abs`, `Math.max`, `IO.println` from loaded stdlib
