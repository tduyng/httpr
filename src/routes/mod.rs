use crate::response::Response;
use crate::Result;

mod not_found;
mod root;

pub use not_found::*;
pub use root::*;

pub async fn handle_routes(path: &str) -> Result<Response> {
    match path {
        "/" => handle_root().await,
        _ => handle_not_found().await,
    }
}
