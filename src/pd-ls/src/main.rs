use anyhow::Result;
use lsp_server::Connection;

fn main() -> Result<()> {
    let (connection, io_threads) = Connection::stdio();
    let (initialize_id, initialize_params) = connection.initialize_start()?;
    let initialize_params =
        serde_json::from_value::<lsp_types::InitializeParams>(initialize_params)?;
    Ok(())
}
