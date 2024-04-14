use crate::{request::RequestContext, response::Response, Result};
use std::{fs, path::Path};

pub async fn get_files(context: &RequestContext<'_>) -> Result<Response> {
    let filename = context.request.path.trim_start_matches("/files/");
    let file_path = Path::new(&context.args.directory).join(filename);

    if file_path.exists() {
        match fs::read(file_path) {
            Ok(contents) => Ok(Response::new()
                .status_code(200, "OK")
                .header("Content-Type", "application/octet-stream")
                .body(&contents)),
            Err(_) => Ok(Response::new()
                .status_code(500, "Internal Server Error")
                .body_str("Failed to read file contents")),
        }
    } else {
        Ok(Response::new()
            .status_code(404, "Not Found")
            .body_str("File not found"))
    }
}

pub async fn post_files(context: &RequestContext<'_>) -> Result<Response> {
    let filename = context.request.path.trim_start_matches("/files/");
    let file_path = Path::new(&context.args.directory).join(filename);
    let file_content = context.request.body.clone();

    match fs::write(file_path, file_content) {
        Ok(_) => Ok(Response::new()
            .status_code(201, "Created")
            .body_str("File uploaded successfully")),
        Err(_) => Ok(Response::new()
            .status_code(500, "Internal Server Error")
            .body_str("Failed to save file")),
    }
}
