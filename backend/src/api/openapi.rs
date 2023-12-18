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
        ..
    }: OpenApiSchemaCommandArguements
) -> Result<(), anyhow::Error>{
    todo!()
}