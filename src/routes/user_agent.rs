use crate::{request::Request, response::Response, Result};

pub async fn get_user_agent(request: &Request) -> Result<Response> {
    let user_agent = match request
        .headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("User-Agent"))
    {
        Some((_, value)) => String::from_utf8_lossy(value).into_owned(),
        None => return Err("User-Agent header not found".into()),
    };

    Ok(Response::new()
        .body_str(&user_agent)
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
