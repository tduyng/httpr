use crate::response::Response;
use crate::Result;

pub async fn not_found() -> Result<Response> {
    Ok(Response::new()
        .body_str("404 Not Found")
        .status_code(404, "Not Found")
        .header("Content-Type", "text/plain"))
}
