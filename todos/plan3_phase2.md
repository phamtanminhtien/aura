# Plan 3 Phase 2: LSP Infrastructure

## 📌 Goals

Build the foundation of the Aura Language Server (LSP) to provide basic IDE support.

## 📝 Tasks

- [ ] Add `tower-lsp` and `tokio` dependencies to `Cargo.toml`
- [ ] Implement basic LSP server in `src/lsp/server.rs`
- [ ] Support `initialize`, `initialized`, `shutdown`, and `exit` notifications
- [ ] Implement `textDocument/didOpen` and `textDocument/didChange` to trigger compilation
- [ ] Integrate `Diagnostics` from the compiler into the LSP
- [ ] Add a CLI flag `--lsp` to `src/main.rs` to start the language server
- [ ] Implement basic "Hover" support (showing type info)
- [ ] Implement basic "Go to Definition" support
