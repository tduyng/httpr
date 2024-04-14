use crate::request::Request;
use crate::response::Response;
use crate::Result;

mod echo;
mod not_found;
mod root;
mod user_agent;

pub use echo::*;
pub use not_found::*;
pub use root::*;
pub use user_agent::*;

pub async fn handle_routes(request: &Request) -> Result<Response> {
    match request.path.to_lowercase().as_str() {
        p if p.starts_with("/echo/") => handle_echo(request).await,
        "/user-agent" => handle_user_agent(request).await,
        "/" => handle_root().await,
        _ => handle_not_found().await,
    }
}
