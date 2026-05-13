import * as path from "path";
import * as vscode from "vscode";
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	const serverExe = process.platform === "win32" ? "lsp.exe" : "lsp";
	const serverPath = context.asAbsolutePath(path.join("server", serverExe));

	const serverOptions: ServerOptions = {
		run: { command: serverPath },
		debug: { command: serverPath, args: ["--debug"] },
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "kohan" }],
	};

	client = new LanguageClient(
		"kohan-ls",
		"kohan Language Server",
		serverOptions,
		clientOptions
	);

	client.start();
	context.subscriptions.push(client);
}

export function deactivate(): Thenable<void> | undefined {
	return client?.stop();
}
