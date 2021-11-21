import * as vscode from "vscode";

export interface Config {
  serverPath: string;
}

export function createConfig(context: vscode.ExtensionContext): Config {
  return {
    serverPath: "pdls",
  };
}
