use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route(
        "/api",
        get(|| async { Html("Hello, <string>World!</string>") }),
    );

    let address = SocketAddr::from(([127, 0, 0, 1], 3010));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);

    axum::serve(listener, app).await.unwrap();
}
