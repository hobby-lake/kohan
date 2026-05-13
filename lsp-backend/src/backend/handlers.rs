use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

use crate::backend::Backend;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "my-lsp".into(),
                version: Some("0.1.0".into()),
            }),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.map.write().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        if let Some(change) = params.content_changes.first() {
            self.documents.map.write().await.insert(uri, change.text.clone());
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let pos = params.text_document_position_params.position;
        let msg = format!("Hover at line {}, char {}", pos.line, pos.character);

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(msg)),
            range: None,
        }))
    }
}
