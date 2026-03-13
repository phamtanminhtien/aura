import * as path from "path";
import * as fs from "fs";
import { workspace, ExtensionContext, commands, window } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Executable,
} from "vscode-languageclient/node";

import * as os from "os";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Create output channel for diagnostics
  const outputChannel = window.createOutputChannel("Aura");
  outputChannel.appendLine("Aura extension is activating...");
  outputChannel.appendLine(`Activation context: ${JSON.stringify({
    extensionPath: context.extensionPath,
    storagePath: context.storagePath,
    globalStoragePath: context.globalStoragePath,
    logPath: context.logUri?.fsPath
  })}`);

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
        clientOptions,
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
    }),
  );

  try {
    outputChannel.appendLine("Starting server-related logic...");
    const serverOptions = getServerOptions(outputChannel);
    const clientOptions = getClientOptions();

    const serverPath = (serverOptions as any).run?.command || (serverOptions as any).command || "unknown";
    outputChannel.appendLine(`Server path resolved to: ${serverPath}`);

    // Create the language client and start the client.
    client = new LanguageClient(
      "auraLanguageServer",
      "Aura Language Server",
      serverOptions,
      clientOptions,
    );

    // Start the client. This will also launch the server
    outputChannel.appendLine("Starting Language Client...");
    client.start();
    outputChannel.appendLine("Aura Language Server activation complete.");
  } catch (e) {
    outputChannel.appendLine(`CRITICAL ERROR during Aura activation: ${e}`);
    if (e instanceof Error) {
        outputChannel.appendLine(`Stack trace: ${e.stack}`);
    }
  }
}

function resolveAuraPath(outputChannel: any): string {
  // 1. Check user configuration
  let serverPath =
    workspace.getConfiguration("aura").get<string>("serverPath") ||
    workspace.getConfiguration("aura").get<string>("aura.serverPath");

  if (serverPath && serverPath.trim() !== "") {
    if (path.isAbsolute(serverPath)) {
      if (fs.existsSync(serverPath)) {
        outputChannel.appendLine(
          `Using configured absolute path: ${serverPath}`,
        );
        return serverPath;
      }
    } else {
      // If it's a relative path or just a command name, we'll try to find it
      outputChannel.appendLine(`Configuration found: ${serverPath}`);
    }
  } else {
    serverPath = "aura";
  }

  // 2. Check if it's in the PATH (handled by child_process eventually, but we check here for logging)
  // We don't use 'which' here to avoid external dependencies, but Executable will handle it.
  // However, we can try to find it in common locations explicitly.

  const homeDir = os.homedir();
  const commonPaths = [
    serverPath, // Try the raw command (rely on PATH)
    path.join(homeDir, ".aura", "bin", "aura"), // Standard install
    path.join(homeDir, ".cargo", "bin", "aura"), // Alternative cargo install
  ];

  // Add project-specific path if workspace is open
  const workspaceFolders = workspace.workspaceFolders;
  if (workspaceFolders && workspaceFolders.length > 0) {
    const rootPath = workspaceFolders[0].uri.fsPath;
    commonPaths.push(path.join(rootPath, "target", "debug", "aura"));
    commonPaths.push(path.join(rootPath, "aura"));
  }

  for (const p of commonPaths) {
    try {
      if (p === serverPath) continue; // Skip the raw command for now
      if (fs.existsSync(p)) {
        outputChannel.appendLine(`Found Aura binary at: ${p}`);
        return p;
      }
    } catch (e) {
      // Ignore errors
    }
  }

  outputChannel.appendLine(`Defaulting to command: ${serverPath}`);
  return serverPath;
}

function getServerOptions(outputChannel: any): ServerOptions {
  // The server is implemented in rust
  // We launch it with the --lsp flag

  const serverPath = resolveAuraPath(outputChannel);

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
