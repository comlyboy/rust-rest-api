use std::net::SocketAddr;

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

mod app;
mod handlers;

#[tokio::main]
async fn main() {
  // Set up structured logging
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

  // Get server address (env override or fallback to localhost:3010)
  let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(3300);
  let address = SocketAddr::from(([127, 0, 0, 1], port));

  // Build app
  let app = app::initialize_app();

  // Start server
  info!("📡 Listening on http://{}/api", address);
  info!("📋 Documentation ad http://{}/api/docs", address);

  let listener = tokio::net::TcpListener::bind(address)
    .await
    .expect("Failed to bind TCP listener");

  axum::serve(listener, app).await.expect("Server crashed unexpectedly!");
}
