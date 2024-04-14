use crate::{request::Request, response::Response, Result};

pub async fn handle_echo(request: &Request) -> Result<Response> {
    let path = &request.path;
    let random_string = path.trim_start_matches("/echo/");

    Ok(Response::new()
        .body_str(random_string)
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
