/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { workspace, ExtensionContext } from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(_context: ExtensionContext) {
  const cfg = workspace.getConfiguration("safety-tool");
  const extraEnv = cfg.get<Record<string, string>>("env", {});

  const workspaceRoot = workspace.workspaceFolders?.[0]?.uri.fsPath;

  const run: Executable = {
    // `safety-lsp.env.SAFETY_LSP` can be set in .vscode/settings.json to point to a specific binary
    command: extraEnv["SAFETY_LSP"] ?? "safety-lsp",
    options: {
      // Run safety-lsp under the first workspace root
      cwd: workspaceRoot,
      // `safety-lsp.env.SP_FILE` can be set in .vscode/settings.json to 
      // point to a tag specification TOML path starting from the workspace root
      env: { SP_DISABLE_CHECK: 1, ...extraEnv },
    },
  };

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const serverOptions: ServerOptions = { run, debug: run };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for rust documents
    documentSelector: [{ scheme: "file", language: "rust" }],
    synchronize: {
      // Notify the server about file changes to '.clientrc files contained in the workspace
      fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
    },
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "safety-tool",
    "safety-tool",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client ? client.stop() : undefined;
}
