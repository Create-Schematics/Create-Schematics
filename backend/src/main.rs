#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use std::process::ExitCode;

use tracing::Level;

pub mod cli;
pub mod api;
pub mod models;
pub mod middleware;
pub mod database;
pub mod error;
pub mod authentication;
pub mod response;
pub mod storage;
pub mod helpers;

#[tokio::main]
async fn main() -> ExitCode {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    
    cli::init().await
}