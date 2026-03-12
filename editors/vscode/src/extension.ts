import * as path from "path";
import * as fs from "fs";
import { workspace, ExtensionContext, commands, window } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Executable,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Create output channel for diagnostics
  const outputChannel = window.createOutputChannel("Aura");
  outputChannel.appendLine("Aura extension is activating...");

  const restartServer = async () => {
    outputChannel.appendLine("Restarting Aura Language Server...");
    if (client) {
      try {
        await client.stop();
      } catch (e) {
        outputChannel.appendLine(`Error stopping client: ${e}`);
      }
    }
    
    // We need to re-evaluate the server path in case it changed
    try {
      const serverOptions = getServerOptions(outputChannel);
      const clientOptions = getClientOptions();
      client = new LanguageClient(
        "auraLanguageServer",
        "Aura Language Server",
        serverOptions,
        clientOptions
      );
      client.start();
      outputChannel.appendLine("Aura Language Server started successfully.");
    } catch (e) {
      outputChannel.appendLine(`Failed to start Aura Language Server: ${e}`);
    }
  };

  // Register the restart command immediately so it's available even if activation fails later
  context.subscriptions.push(
    commands.registerCommand("aura.restartServer", restartServer),
    workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration("aura.serverPath")) {
        restartServer();
      }
    })
  );

  try {
    const serverOptions = getServerOptions(outputChannel);
    const clientOptions = getClientOptions();

    // Create the language client and start the client.
    client = new LanguageClient(
      "auraLanguageServer",
      "Aura Language Server",
      serverOptions,
      clientOptions
    );

    // Start the client. This will also launch the server
    client.start();
    outputChannel.appendLine("Aura Language Server activation complete.");
  } catch (e) {
    outputChannel.appendLine(`Error during Aura activation: ${e}`);
  }
}

function getServerOptions(outputChannel: any): ServerOptions {
  // The server is implemented in rust
  // We launch it with the --lsp flag

  // For development, we assume the binary is in the workspace root or built via cargo
  let serverPath = workspace.getConfiguration("aura").get<string>("serverPath") || workspace.getConfiguration("aura").get<string>("aura.serverPath");

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

  outputChannel.appendLine(`Using server path: ${serverPath}`);

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

  return {
    run,
    debug: run,
  };
}

function getClientOptions(): LanguageClientOptions {
  return {
    // Register the server for aura files
    documentSelector: [{ scheme: "file", language: "aura" }],
    synchronize: {
      // Notify the server about file changes to '.aura' files contained in the workspace
      fileEvents: workspace.createFileSystemWatcher("**/*.aura"),
    },
  };
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
