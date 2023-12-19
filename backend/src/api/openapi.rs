use std::fs::File;
use std::io::Write;

use clap::Args;

#[derive(Args, Debug)]
pub struct OpenApiSchemaCommandArguements {
    #[arg(help = "Weather to output a yaml schema")]
    #[arg(short = 'y', long = "yaml")]
    #[arg(default_value = "true")]
    pub yaml: bool, 

    #[arg(help = "Weather to output a json schema")]
    #[arg(short = 'j', long = "json")]
    #[arg(default_value = "true")]
    pub json: bool, 
}

pub fn save_schema(
    OpenApiSchemaCommandArguements {
        yaml,
        json
    }: OpenApiSchemaCommandArguements
) -> Result<(), anyhow::Error>{
    let service = super::build_openapi_service();

    if json {
        let mut output = File::create("openapi.json")?;
        let schema = service.spec();

        output.write_all(schema.as_bytes())?;
    }

    if yaml {
        let mut output = File::create("openapi.yaml")?;
        let schema = service.spec_yaml();

        output.write_all(schema.as_bytes())?;
    }

    Ok(())
}