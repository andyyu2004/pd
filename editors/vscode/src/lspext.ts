import * as lc from "vscode-languageclient";

export interface SyntaxTreeParams {
  textDocument: lc.TextDocumentIdentifier;
  range?: lc.Range;
}

export const syntaxTree = new lc.RequestType<SyntaxTreeParams, string, void>(
  "pdls/syntaxTree"
);
