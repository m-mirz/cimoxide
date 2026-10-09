// Starts `cimlsp` and connects it to XML documents. The server decides which
// documents are CGMES / NC (by namespace) and ignores the rest.

import * as fs from "fs";
import * as path from "path";
import { commands, ExtensionContext, window, workspace } from "vscode";
import { LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";

let client: LanguageClient | undefined;

/** The `cimoxide.server.path` setting, else the bundled binary, else `PATH`. */
function serverCommand(context: ExtensionContext): string {
  const configured = workspace.getConfiguration("cimoxide").get<string>("server.path");
  if (configured) {
    return configured;
  }
  const exe = process.platform === "win32" ? "cimlsp.exe" : "cimlsp";
  const bundled = context.asAbsolutePath(path.join("server", exe));
  if (!fs.existsSync(bundled)) {
    return exe;
  }
  if (process.platform !== "win32") {
    // A VSIX is a zip; not every unpacker keeps the executable bit.
    try {
      fs.chmodSync(bundled, 0o755);
    } catch {
      // Read-only install: run it as it is.
    }
  }
  return bundled;
}

async function start(context: ExtensionContext): Promise<void> {
  const cfg = workspace.getConfiguration("cimoxide");
  const env: NodeJS.ProcessEnv = { ...process.env };
  const rdfs = cfg.get<string>("schema.rdfsDir");
  const shacl = cfg.get<string>("schema.shaclDir");
  if (rdfs) {
    env.CIMOXIDE_RDFS_DIR = rdfs;
  }
  if (shacl) {
    env.CIMOXIDE_SHACL_DIR = shacl;
  }

  const command = serverCommand(context);
  const serverOptions: ServerOptions = { command, options: { env } };
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "xml" }],
    initializationOptions: {
      common: cfg.get<boolean>("validation.common"),
      quality: cfg.get<boolean>("validation.quality"),
      silence: cfg.get<string[]>("validation.silence"),
      onType: cfg.get<boolean>("validation.onType"),
    },
  };
  client = new LanguageClient("cimoxide", "cimoxide", serverOptions, clientOptions);
  try {
    await client.start();
  } catch (e) {
    client = undefined;
    window.showErrorMessage(`cimoxide: could not start the language server (${command}): ${e}`);
  }
}

async function stop(): Promise<void> {
  const c = client;
  client = undefined;
  await c?.stop();
}

export async function activate(context: ExtensionContext): Promise<void> {
  context.subscriptions.push(
    commands.registerCommand("cimoxide.restartServer", async () => {
      await stop();
      await start(context);
    }),
    // Settings are read at start-up (schema directories must be in the
    // server's environment before it loads a table), so any change restarts.
    workspace.onDidChangeConfiguration(async (e) => {
      if (e.affectsConfiguration("cimoxide") && !e.affectsConfiguration("cimoxide.trace")) {
        await stop();
        await start(context);
      }
    }),
  );
  await start(context);
}

export function deactivate(): Promise<void> {
  return stop();
}
