import { LanguageClient } from "vscode-languageclient/node";
import * as vscode from "vscode";
import { createClient } from "./client";
import { Config } from "./config";

export interface LspContext {
  registerCommand(name: string, f: (cx: LspContext) => Cmd): Promise<void>;
  client: LanguageClient;
}

export type Cmd = (...args: any[]) => unknown;

export async function createContext(
  context: vscode.ExtensionContext,
  { serverPath }: Config
): Promise<LspContext> {
  console.debug("starting pdls client");
  const client = createClient(serverPath);
  context.subscriptions.push(client.start());
  await client.onReady();
  console.debug("LSP client ready");

  const lcx = {
    client,
    registerCommand,
  };

  async function registerCommand(name: string, f: (cx: LspContext) => Cmd) {
    const d = vscode.commands.registerCommand(`pd-lsp.${name}`, f(lcx));
    context.subscriptions.push(d);
  }

  return lcx;
}
