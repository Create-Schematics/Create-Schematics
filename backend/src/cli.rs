use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::api;
use crate::api::openapi::OpenApiSchemaCommandArguements;
use crate::api::StartCommandServerArguments;

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
    Start(StartCommandServerArguments), 

    #[command(name = "openapi-schema")]
    Openapi(OpenApiSchemaCommandArguements)
}

pub async fn init() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start(args) => api::serve(args).await,
        Commands::Openapi(args) => api::openapi::save_schema(args),
    };
        
    if let Err(e) = result {
        tracing::error!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}