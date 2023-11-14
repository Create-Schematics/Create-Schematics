use std::process::ExitCode;

pub mod cli;
pub mod api;
pub mod models;
pub mod database;
pub mod response;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    cli::init().await
}
