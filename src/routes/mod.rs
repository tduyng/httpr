use crate::request::RequestContext;
use crate::response::Response;
use crate::Result;

mod echo;
mod files;
mod not_found;
mod root;
mod user_agent;

pub use echo::*;
pub use files::*;
pub use not_found::*;
pub use root::*;
pub use user_agent::*;

pub async fn handle_routes(context: &RequestContext<'_>) -> Result<Response> {
    match (
        context.request.method.as_str(),
        context.request.path.to_lowercase().as_str(),
    ) {
        ("GET", p) if p.starts_with("/echo/") => get_echo(context.request).await,
        ("GET", p) if p.starts_with("/files/") => get_files(context).await,
        ("GET", "/user-agent") => get_user_agent(context.request).await,
        ("GET", "/") => get_root().await,
        ("POST", p) if p.starts_with("/files/") => post_files(context).await,
        _ => not_found().await,
    }
}
