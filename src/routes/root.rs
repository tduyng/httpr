use crate::error::ServerError;
use crate::response::Response;

pub async fn get_root() -> Result<Response, ServerError> {
    Ok(Response::new()
        .body_str("Hello World!")
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
