# Plan 3 Phase 2: LSP Infrastructure

## 📌 Goals

Build the foundation of the Aura Language Server (LSP) to provide basic IDE support.

## 📝 Tasks

- [x] Add `tower-lsp` and `tokio` dependencies to `Cargo.toml`
- [x] Implement basic LSP server in `src/lsp/server.rs`
- [x] Support `initialize`, `initialized`, `shutdown`, and `exit` notifications
- [x] Implement `textDocument/didOpen` and `textDocument/didChange` to trigger compilation
- [x] Integrate `Diagnostics` from the compiler into the LSP
- [x] Add a CLI flag `--lsp` to `src/main.rs` to start the language server
- [x] Implement basic "Hover" support (showing type info)
- [x] Implement basic "Go to Definition" support
