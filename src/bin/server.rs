use anyhow::Result;
use rhhtp::{Context, Router, Server, StatusCode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new();
    let mut router = Router::new();

    async fn hello_world_handler(ctx: &mut Context) {
        let resp = b"<h1>Hello World</h1>";
        let mut response = ctx.response.lock().await;
        response.content_type("text/html");
        response.write_body(resp);
    }

    async fn hello_name_handler(ctx: &mut Context) {
        let mut response = ctx.response.lock().await;
        response.content_type("text/html");
        response.write_body(b"<h1>Hello ");

        let params = &ctx.path_params;
        let name = params.get(":name").map_or(b"World" as &[u8], |name| name.as_bytes());

        response.write_body(name);
        response.write_body(b"</h1>");
    }

    async fn not_found_handler(ctx: &mut Context) {
        let resp = b"404";
        let mut response = ctx.response.lock().await;
        response.status_code(StatusCode::NotFound);
        response.write_body(resp);
    }

    router.get("/", hello_world_handler);
    router.get("/:name", hello_name_handler);
    router.any("*", not_found_handler);

    server.apply(router);

    let address: SocketAddr = "[::1]:2024".parse()?;
    server.listen(address).await
}
