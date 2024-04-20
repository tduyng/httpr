use crate::{error::ServerError, request::Request, response::Response};

pub async fn get_user_agent(request: &Request) -> Result<Response, ServerError> {
    println!("UserAgent request: {:?}", request);
    println!("Headers request: {:?}", request.headers);
    let user_agent = match request
        .headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("User-Agent"))
    {
        Some((_, value)) => value,
        None => "",
    };

    println!("user_agent: {:?}", user_agent);

    Ok(Response::new()
        .body_str(user_agent)
        .status_code(200, "OK")
        .header("Content-Type", "text/plain"))
}
