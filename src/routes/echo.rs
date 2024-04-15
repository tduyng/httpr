use crate::{error::ServerError, request::Request, response::Response};

pub async fn get_echo(request: &Request) -> Result<Response, ServerError> {
    let path = &request.path;
    let random_string = path.trim_start_matches("/echo/");

    Ok(Response::new()
        .body_str(random_string)
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
