import * as vscode from "vscode";
import * as commands from "./commands";
import { createConfig } from "./config";
import { createContext } from "./context";

export async function activate(context: vscode.ExtensionContext) {
  tryActivate(context).catch(err => {
    void vscode.window.showErrorMessage(
      `Cannot activate pd-lsp: ${err.message}`
    );
    throw err;
  });
}

export async function tryActivate(context: vscode.ExtensionContext) {
  const config = createConfig(context);
  const lcx = await createContext(context, config);
  commands.register(lcx);
}

export function deactivate() {}
