// Starts `cimlsp` and connects it to XML documents. The server decides which
// documents are CGMES / NC (by namespace) and ignores the rest.
//
// Also offers `cimmcp` to chat as an MCP server, reading the model sets of
// the first workspace folder.

import * as fs from "fs";
import * as path from "path";
import {
  commands,
  EventEmitter,
  ExtensionContext,
  lm,
  McpStdioServerDefinition,
  window,
  workspace,
} from "vscode";
import { LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";

let client: LanguageClient | undefined;

/** The `cimoxide.<setting>` path, else the bundled `name` binary, else `PATH`. */
function serverCommand(context: ExtensionContext, name: string, setting: string): string {
  const configured = workspace.getConfiguration("cimoxide").get<string>(setting);
  if (configured) {
    return configured;
  }
  const exe = process.platform === "win32" ? `${name}.exe` : name;
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

/** The schema directories, as the servers read them from their environment. */
function schemaEnv(): Record<string, string> {
  const cfg = workspace.getConfiguration("cimoxide");
  const env: Record<string, string> = {};
  const rdfs = cfg.get<string>("schema.rdfsDir");
  const shacl = cfg.get<string>("schema.shaclDir");
  if (rdfs) {
    env.CIMOXIDE_RDFS_DIR = rdfs;
  }
  if (shacl) {
    env.CIMOXIDE_SHACL_DIR = shacl;
  }
  return env;
}

async function start(context: ExtensionContext): Promise<void> {
  const cfg = workspace.getConfiguration("cimoxide");
  const env: NodeJS.ProcessEnv = { ...process.env, ...schemaEnv() };
  const command = serverCommand(context, "cimlsp", "server.path");
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

/** Offers `cimmcp` to chat; VS Code starts it when a chat first uses it. */
function registerMcp(context: ExtensionContext): void {
  const changed = new EventEmitter<void>();
  context.subscriptions.push(
    changed,
    lm.registerMcpServerDefinitionProvider("cimoxide", {
      onDidChangeMcpServerDefinitions: changed.event,
      provideMcpServerDefinitions: () => {
        if (!workspace.getConfiguration("cimoxide").get<boolean>("mcp.enabled", true)) {
          return [];
        }
        const folder = workspace.workspaceFolders?.[0]?.uri.fsPath;
        const command = serverCommand(context, "cimmcp", "mcp.path");
        const args = folder ? ["--dir", folder] : [];
        const version = context.extension.packageJSON.version as string;
        return [new McpStdioServerDefinition("cimoxide", command, args, schemaEnv(), version)];
      },
    }),
    workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration("cimoxide.mcp") || e.affectsConfiguration("cimoxide.schema")) {
        changed.fire();
      }
    }),
    workspace.onDidChangeWorkspaceFolders(() => changed.fire()),
  );
}

export async function activate(context: ExtensionContext): Promise<void> {
  registerMcp(context);
  context.subscriptions.push(
    commands.registerCommand("cimoxide.restartServer", async () => {
      await stop();
      await start(context);
    }),
    // Settings are read at start-up (schema directories must be in the
    // server's environment before it loads a table), so any change restarts.
    workspace.onDidChangeConfiguration(async (e) => {
      if (
        e.affectsConfiguration("cimoxide") &&
        !e.affectsConfiguration("cimoxide.trace") &&
        !e.affectsConfiguration("cimoxide.mcp")
      ) {
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
