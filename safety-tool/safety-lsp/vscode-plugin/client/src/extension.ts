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
  const run: Executable = {
    command: "safety-lsp",
    options: { env: { SP_DISABLE_CHECK: 1 } },
  };

  const debug: Executable = {
    command: "safety-lsp",
    options: {
      env: {
        SP_DISABLE_CHECK: 1,
        SP_FILE: "A:\\Rust\\tag-std\\safety-tool\\assets\\sp-core.toml",
      },
    },
  };

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const serverOptions: ServerOptions = { run, debug };

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
