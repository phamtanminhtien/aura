# Aura Support

Visual Studio Code extension for the **Aura** programming language.

Aura is a modern, high-performance programming language. This extension provides a premium development experience with syntax highlighting, language server integration, and more.

## Features

- ✨ **Syntax Highlighting**: Beautiful and accurate syntax highlighting for `.aura` files.
- 🚀 **Language Server Integration**: Advanced features via the Aura Language Server, including:
  - Code completion
  - Go to definition
  - Error diagnostics
  - Formatting
- 📁 **File Icons**: Custom Aura file icons for better project navigation.
- 🔄 **Restart Command**: Quickly restart the language server when needed.

## Configuration

This extension provides the following settings:

- `aura.serverPath`: Path to the Aura language server binary. If not specified, the extension will attempt to find it in your `PATH` or in `target/debug/aura` relative to the workspace root.

## Commands

- `Aura: Restart Language Server`: Manually restarts the Aura Language Server.

## Development

If you are developing the Aura extension:

1. Clone the repository.
2. Open the `editors/vscode` directory in VS Code.
3. Run `npm install` to install dependencies.
4. Press `F5` to open a new window with the extension loaded.

---

Built with ❤️ by the Aura team.
