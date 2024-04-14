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
    match context.request.path.to_lowercase().as_str() {
        p if p.starts_with("/echo/") => get_echo(context.request).await,
        p if p.starts_with("/files/") => get_files(context).await,
        "/user-agent" => get_user_agent(context.request).await,
        "/" => get_root().await,
        _ => not_found().await,
    }
}
