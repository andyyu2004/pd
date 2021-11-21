import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

export function createClient(serverPath: string): LanguageClient {
  const clientOptions: LanguageClientOptions = {};

  const run: Executable = {
    command: serverPath,
  };

  const serverOptions: ServerOptions = {
    run,
    debug: run,
  };

  const client = new LanguageClient(
    "pdls",
    "pd language server",
    serverOptions,
    clientOptions
  );

  return client;
}
