use crate::{error::ServerError, request::Request, response::Response};

pub async fn get_user_agent(request: &Request) -> Result<Response, ServerError> {
    let user_agent = match request
        .headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("User-Agent"))
    {
        Some((_, value)) => String::from_utf8_lossy(value).into_owned(),
        None => return Err(ServerError::NotFound),
    };

    Ok(Response::new()
        .body_str(&user_agent)
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
