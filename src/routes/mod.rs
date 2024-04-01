use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::request::{HttpRequest, RequestMethod};

use self::{
    echo::handle_echo_request,
    files::{handle_file_request, handle_post_file_request},
    not_found::handle_not_found_request,
    root::handle_root_request,
    user_agent::handle_user_agent_request,
};

pub mod echo;
pub mod files;
pub mod not_found;
pub mod root;
pub mod user_agent;

pub async fn handle_request(mut stream: TcpStream, directory: &str) {
    if let Some(http_request) = HttpRequest::parse_request(&mut stream).await {
        let response = match http_request.method {
            RequestMethod::Get => match http_request.path.as_str() {
                path if path.starts_with("/echo/") => handle_echo_request(path).await,
                path if path.starts_with("/files/") => handle_file_request(directory, path).await,
                "/user-agent" => handle_user_agent_request(&http_request.headers).await,
                "/" => handle_root_request().await,
                _ => handle_not_found_request().await,
            },
            RequestMethod::Post => match http_request.path.as_str() {
                path if path.starts_with("/files/") => {
                    handle_post_file_request(directory, path, http_request.body).await
                }
                _ => handle_not_found_request().await,
            },
        };

        if let Err(e) = stream
            .write_all(&response.into_response_string().into_bytes())
            .await
        {
            eprintln!("Failed to write HttpResponse: {}", e);
        }
    }
}
