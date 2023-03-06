import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import * as path from "path";

export async function deactivate() { }

export interface KIDLExtensionApi {
  readonly client?: lc.LanguageClient;
}

export async function activate(
  context: vscode.ExtensionContext
): Promise<KIDLExtensionApi> {
  let client = new lc.LanguageClient(
    "kidl-cli",
    {
      command:
        process.env.__KIDL_LSP_SERVER ||
        path.join(__dirname, "kidl-cli"),
      args: ["lsp"],
    },
    {
      documentSelector: [{ language: "kidl" }],
    }
  );

  client.start();

  return {
    client,
  };
}
