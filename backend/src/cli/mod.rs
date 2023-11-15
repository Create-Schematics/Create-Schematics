use std::process::ExitCode;

use clap::{Parser, Subcommand};

use self::server::StartCommandServerArguments;

pub mod server;

#[derive(Parser, Debug)]
#[command(name = "Create schematics command line interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "server")]
    Start(StartCommandServerArguments)   
}

pub async fn init() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start(args) => server::init(args).await,
    };
        
    if let Err(e) = result {
        tracing::error!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}