#[macro_use]
extern crate tracing;

mod capabilities;
mod context;
mod dispatch;

use anyhow::Result;
use context::LspContext;
use lsp_server::Connection;
use lsp_types::notification::Notification;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    info!("starting pdls");
    let (connection, io_threads) = Connection::stdio();
    let (initialize_id, initialize_params) = connection.initialize_start()?;
    let _initialize_params =
        serde_json::from_value::<lsp_types::InitializeParams>(initialize_params)?;

    let capabilities = capabilities::caps();
    let initialize_result = lsp_types::InitializeResult { capabilities, ..Default::default() };

    connection.initialize_finish(initialize_id, serde_json::to_value(initialize_result)?)?;

    info!("pdls initialized");

    let mut lcx = LspContext::new();
    while let Some(event) = lcx.next_event(&connection.receiver) {
        if let Event::Lsp(lsp_server::Message::Notification(notif)) = &event {
            if notif.method == lsp_types::notification::Exit::METHOD {
                return Ok(());
            }
        }
        lcx.handle_event(event)?;
    }

    info!("exited event loop");

    io_threads.join()?;
    info!("pdls terminated");
    Ok(())
}

#[derive(Debug)]
enum Event {
    Lsp(lsp_server::Message),
}
