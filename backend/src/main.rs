use std::process::ExitCode;

use tracing::Level;
use backend::cli;

#[tokio::main]
async fn main() -> ExitCode {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    
    cli::init().await
}