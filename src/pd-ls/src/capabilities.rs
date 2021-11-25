use lsp_types::{
    OneOf, SaveOptions, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions
};

pub(crate) fn caps() -> ServerCapabilities {
    lsp_types::ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::INCREMENTAL),
            will_save: None,
            will_save_wait_until: None,
            save: Some(SaveOptions::default().into()),
        })),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}
