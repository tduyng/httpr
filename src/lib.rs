use clap::Parser;

pub mod request;
pub mod response;
pub mod routes;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug, Clone)]
pub struct CliArgs {
    #[clap(short, long, default_value = "./")]
    pub directory: String,
}
