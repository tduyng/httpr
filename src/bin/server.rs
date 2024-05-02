use anyhow::Result;
use rhhtp::{handler, Router, Server, StatusCode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new();
    let address: SocketAddr = "[::1]:2024".parse()?;

    let hello_world_handler = handler!(|ctx| {
        let resp = b"<h1>Hello World</h1>";
        ctx.response.content_type("text/html");
        ctx.response.write_body(resp);
        ctx.end();
    });

    let hello_name_handler = handler!(|ctx| {
        ctx.response.content_type("text/html");
        ctx.response.write_body(b"<h1>Hello ");

        let params = ctx.params.clone();
        let name = params.get(":name");
        let name = if let Some(name) = name {
            name.value.as_bytes()
        } else {
            b"World"
        };

        ctx.response.write_body(name);
        ctx.response.write_body(b"</h1>");
        ctx.end();
    });

    server
        .get("/", hello_world_handler)
        .await
        .get("/:name", hello_name_handler)
        .await;

    server
        .any(
            "*",
            handler!(|ctx| {
                let resp = b"404";
                ctx.response.status_code(StatusCode::NotFound);
                ctx.response.write_body(resp);
            }),
        )
        .await;

    server.listen(address).await
}
