use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CliArgs {
    #[clap(short, long, default_value = "./")]
    pub directory: String,
}
