import * as path from "path";
import * as fs from "fs";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Executable,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // The server is implemented in rust
  // We launch it with the --lsp flag

  // For development, we assume the binary is in the workspace root or built via cargo
  // You might want to allow configuring this path in settings
  let serverPath = workspace.getConfiguration("aura").get<string>("serverPath");

  if (!serverPath || serverPath === "aura" || serverPath === "aura-dev") {
    // If not explicitly set to an absolute path, try to find it
    const workspaceFolder = workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (workspaceFolder) {
      // 1. Try target/debug/aura in current workspace
      const localPath = path.join(workspaceFolder, "target", "debug", "aura");
      if (fs.existsSync(localPath)) {
        serverPath = localPath;
      } else {
        // 2. Try sibling aura-rust/target/debug/aura (common for projects sibling to the compiler)
        const siblingPath = path.join(
          workspaceFolder,
          "..",
          "aura-rust",
          "target",
          "debug",
          "aura"
        );
        if (fs.existsSync(siblingPath)) {
          serverPath = siblingPath;
        } else {
          // Fallback to "aura" in PATH
          serverPath = "aura";
        }
      }
    } else {
      serverPath = "aura";
    }
  }

  const run: Executable = {
    command: serverPath,
    args: ["--lsp"],
    options: {
      env: {
        ...process.env,
        RUST_LOG: "debug",
      },
    },
  };

  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for aura files
    documentSelector: [{ scheme: "file", language: "aura" }],
    synchronize: {
      // Notify the server about file changes to '.aura' files contained in the workspace
      fileEvents: workspace.createFileSystemWatcher("**/*.aura"),
    },
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "auraLanguageServer",
    "Aura Language Server",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
