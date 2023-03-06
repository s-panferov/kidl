use std::error::Error;

use lsp_server::{Connection, ExtractError, Message, Request, Response, ResponseError};
use lsp_types::{InitializeParams, PositionEncodingKind, ServerCapabilities};

pub fn start() -> Result<(), Box<dyn Error + Sync + Send>> {
    tracing::info!("KIDL LSP server is ready");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF16),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

pub fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");
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
