import { LanguageClient } from "vscode-languageclient/node";
import * as vscode from "vscode";
import { createClient } from "./client";
import { Config } from "./config";

export interface LspContext {
  registerCommand(name: string, f: (cx: LspContext) => Cmd): Promise<void>;
  activeEditor(): PdEditor | undefined;
  disposable(disposable: vscode.Disposable): void;
  subscriptions: vscode.Disposable[];
  client: LanguageClient;
}

export interface PdDocument extends vscode.TextDocument {}

export interface PdEditor extends vscode.TextEditor {
  document: PdDocument;
}

export function isPdDocument(
  document: vscode.TextDocument
): document is PdDocument {
  return document.languageId === "pd" && document.uri.scheme === "file";
}

export function isPdEditor(editor: vscode.TextEditor): editor is PdEditor {
  return isPdDocument(editor.document);
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

  const lcx: LspContext = {
    client,
    subscriptions: context.subscriptions,
    disposable,
    registerCommand,
    activeEditor,
  };

  function disposable(disposable: vscode.Disposable) {
    context.subscriptions.push(disposable);
  }

  function activeEditor(): PdEditor | undefined {
    const editor = vscode.window.activeTextEditor;
    return editor && isPdEditor(editor) ? editor : undefined;
  }

  async function registerCommand(name: string, f: (cx: LspContext) => Cmd) {
    const d = vscode.commands.registerCommand(`pd-lsp.${name}`, f(lcx));
    disposable(d);
  }

  return lcx;
}
