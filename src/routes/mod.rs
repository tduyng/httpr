use crate::error::{not_found, ServerError};
use crate::request::RequestContext;
use crate::response::Response;

mod echo;
mod files;
mod root;
mod user_agent;

pub use echo::*;
pub use files::*;
pub use root::*;
pub use user_agent::*;

pub async fn handle_routes(context: &RequestContext<'_>) -> Result<Response, ServerError> {
    println!("path: {}", context.request.path.to_lowercase());

    match (
        context.request.method.as_str(),
        context.request.path.to_lowercase().as_str(),
    ) {
        ("GET", p) if p.starts_with("/echo/") => get_echo(context.request).await,
        ("GET", p) if p.starts_with("/files/") => get_files(context).await,
        ("GET", "/user-agent") => get_user_agent(context.request).await,
        ("GET", "/") => get_root().await,
        ("POST", p) if p.starts_with("/files/") => post_files(context).await,
        _ => Ok(not_found()),
    }
}
