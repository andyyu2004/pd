import { Cmd, isPdDocument, LspContext } from "./context";
import * as vscode from "vscode";
import * as lspext from "./lspext";
import { ProviderResult, Uri, workspace } from "vscode";

export function register(lcx: LspContext) {
  lcx.registerCommand("status", status);
  lcx.registerCommand("syntax-tree", showSyntaxTree);
}

export function status(lcx: LspContext): Cmd {
  return () => {
    console.log("TODO status");
  };
}

const showSyntaxTree = (lcx: LspContext) => async () => {
  const editor = lcx.activeEditor();
  if (!editor) {
    return;
  }

  const astUri = vscode.Uri.parse("pdls://syntaxtree/tree.rast");
  const emitter = new vscode.EventEmitter<vscode.Uri>();
  const tdcp: vscode.TextDocumentContentProvider = {
    onDidChange: emitter.event,
    provideTextDocumentContent(
      _uri: Uri,
      token: vscode.CancellationToken
    ): ProviderResult<string> {
      const params: lspext.SyntaxTreeParams = {
        textDocument: { uri: editor.document.uri.toString() },
      };
      return lcx.client.sendRequest(lspext.syntaxTree, params, token);
    },
  };

  vscode.workspace.onDidChangeTextDocument(
    e => isPdDocument(e.document) && emitter.fire(astUri),
    tdcp,
    lcx.subscriptions
  );
  vscode.window.onDidChangeActiveTextEditor(
    e => e && isPdDocument(e.document) && emitter.fire(astUri),
    tdcp,
    lcx.subscriptions
  );

  lcx.disposable(workspace.registerTextDocumentContentProvider("pdls", tdcp));

  const document = await vscode.workspace.openTextDocument(astUri);

  await vscode.window.showTextDocument(document, {
    viewColumn: vscode.ViewColumn.Two,
    preserveFocus: true,
  });
};
