use std::path::Path;

use kidl_db::Database;
use lsp_types::{
    DidOpenTextDocumentParams, SaveOptions, TextDocumentContentChangeEvent,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, Url,
};
use ropey::Rope;

use crate::position::position_to_char;

pub(crate) fn capabilities() -> TextDocumentSyncCapability {
    TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
        open_close: Some(true),
        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
            include_text: Some(false),
        })),
        change: Some(TextDocumentSyncKind::INCREMENTAL),
        ..Default::default()
    })
}

pub(crate) fn edit(db: &mut Database, uri: &Url, edits: Vec<TextDocumentContentChangeEvent>) {
    let path = Path::new(uri.path());

    tracing::info!(path = ?path, "Applying changes to the document");
    match db.schema_file(path) {
        Some(schema_file) => {
            let mut buffer = schema_file.text(db);
            for edit in edits {
                match edit.range {
                    Some(range) => {
                        tracing::info!("Applying change: {:#?}", edit);

                        let start_char = position_to_char(&buffer, range.start);
                        let end_char = position_to_char(&buffer, range.end);

                        buffer.remove(start_char..end_char);
                        buffer.insert(start_char, &edit.text);
                    }
                    None => buffer = Rope::from(edit.text),
                }
            }

            schema_file.set_text(db).to(buffer);
        }
        None => {
            assert_eq!(edits.len(), 1);
            let change = edits.into_iter().next().unwrap();
            assert!(change.range.is_none());
            db.push_file(path.to_owned(), change.text);
        }
    }
}

pub(crate) fn open(db: &mut Database, params: &DidOpenTextDocumentParams) {
    let path = Path::new(params.text_document.uri.path());
    let source = std::fs::read_to_string(path).unwrap();
    if let Some(schema_file) = db.schema_file(path) {
        tracing::info!(path = ?path, "Re-Open existing document");
        schema_file.set_text(db).to(Rope::from(source));
    } else {
        tracing::info!(path = ?path, "Open new document");
        db.push_file(path.to_owned(), source);
    }
}
