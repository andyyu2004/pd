import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

export function createClient(serverPath: string): LanguageClient {
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "pd" }],
  };

  const run: Executable = {
    command: serverPath,
    options: {
      env: {
        ...process.env,
        // eslint-disable-next-line @typescript-eslint/naming-convention
        RUST_LOG: "TRACE",
      },
    },
  };

  const debug = run;
  const serverOptions: ServerOptions = {
    run,
    debug,
  };

  const client = new LanguageClient(
    "pdls",
    "pd language server",
    serverOptions,
    clientOptions
  );

  return client;
}
