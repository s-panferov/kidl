use std::error::Error;

use kidl_db::Database;

use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, Response, ResponseError,
};

use lsp_types::{
    notification::{DidChangeTextDocument, DidOpenTextDocument, Notification as _},
    InitializeParams, PositionEncodingKind, ServerCapabilities,
};

pub mod position;
pub mod text;

pub fn start() -> Result<(), Box<dyn Error + Sync + Send>> {
    tracing::info!("KIDL LSP server is ready");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF16),
        text_document_sync: Some(self::text::capabilities()),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    eprintln!("shutting down server");

    Ok(())
}

pub fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");

    let mut db = Database::default();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                eprintln!("got request: {:?}", req.method);

                let id = req.id.clone();

                let result = match AsRef::<str>::as_ref(&req.method) {
                    _ => Err(ResponseError {
                        code: 0,
                        message: String::from("Unknown request"),
                        data: None,
                    }),
                };

                match result {
                    Ok(result) => {
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(e) => {
                        let resp = Response {
                            id,
                            result: None,
                            error: Some(e),
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                }
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => match AsRef::<str>::as_ref(&not.method) {
                DidOpenTextDocument::METHOD => {
                    let params = cast_notification::<DidOpenTextDocument>(not).unwrap();
                    crate::text::open(&mut db, &params);
                }
                DidChangeTextDocument::METHOD => {
                    let params = cast_notification::<DidChangeTextDocument>(not).unwrap();
                    crate::text::edit(&mut db, &params.text_document.uri, params.content_changes);
                }
                _ => {
                    eprintln!("got notification: {:?}", not.method);
                }
            },
        }
    }
    Ok(())
}

pub fn to_response_error(err: ExtractError<Request>) -> ResponseError {
    ResponseError {
        code: 0,
        message: err.to_string(),
        data: None,
    }
}

pub fn answer<R>(
    req: Request,
    func: impl FnOnce(R::Params) -> R::Result,
) -> Result<serde_json::Value, ResponseError>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
        .map(|(_, params)| serde_json::to_value(&func(params)).unwrap())
        .map_err(to_response_error)
}

fn cast_notification<N>(req: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    req.extract(N::METHOD)
}
