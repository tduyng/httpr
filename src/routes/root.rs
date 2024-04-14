use crate::response::Response;
use crate::Result;

pub async fn handle_root() -> Result<Response> {
    Ok(Response::new()
        .body_str("Hello World!")
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
