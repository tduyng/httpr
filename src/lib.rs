use clap::Parser;

pub mod error;
pub mod request;
pub mod response;
pub mod routes;

#[derive(Parser, Debug, Clone)]
pub struct CliArgs {
    #[clap(short, long, default_value = "./")]
    pub directory: String,
}
