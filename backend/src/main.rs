#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use std::process::ExitCode;

pub mod cli;
pub mod api;
pub mod models;
pub mod database;
pub mod error;
pub mod response;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    cli::init().await
}
