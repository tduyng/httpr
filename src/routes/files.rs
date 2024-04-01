use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use tokio::{
    fs::{self, create_dir_all, File},
    io::AsyncReadExt,
};

use crate::response::{ContentType, HttpResponse};

use super::not_found::handle_not_found_request;

pub async fn handle_file_request(directory: &str, path: &str) -> HttpResponse {
    let filename = path.trim_start_matches("/files/");
    let mut file_path = PathBuf::from(directory);
    file_path.push(filename);

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents).await {
                Ok(_) => HttpResponse {
                    protocol: "HTTP/1.1".to_string(),
                    status_code: 200,
                    status_text: "OK".to_string(),
                    body: Some(contents),
                    headers: HashMap::new(),
                    content_type: ContentType::ApplicationOctetStream,
                },
                Err(_) => handle_not_found_request().await,
            }
        }
        Err(_e) => handle_not_found_request().await,
    }
}

pub async fn handle_post_file_request(directory: &str, path: &str, body: String) -> HttpResponse {
    let filename = path.trim_start_matches("/files/");
    let file_path = Path::new(&directory).join(filename);

    if !file_path.parent().unwrap().exists() {
        if let Err(err) = create_dir_all(file_path.parent().unwrap()).await {
            eprintln!("Error creating parent directories: {}", err);
        }
    }

    if !file_path.exists() {
        if let Err(err) = File::create(&file_path).await {
            eprintln!("Creating new file error: {} - {:?}", err, file_path);
        }
    }

    if let Err(err) = fs::write(&file_path, body).await {
        eprintln!("Error writing file: {}", err);
    }

    HttpResponse {
        protocol: "HTTP/1.1".to_string(),
        status_code: 201,
        status_text: "Created".to_string(),
        body: None,
        headers: HashMap::new(),
        content_type: ContentType::None,
    }
}
