use axum::{
  Router,
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::{get, post},
};

use mongodb::Database;

use crate::{config::AppConfig, database::initialize_dbs};
use crate::{database::Databases, handlers};

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
  pub env: String,
  pub databases: Databases,
}

/** Initialise API */
pub async fn initialize_app() -> Router {
  // Initialize database
  let databases = initialize_dbs().await;

  let state = AppState {
    env: AppConfig::from_env().env,
    databases: databases,
  };

  return Router::new().nest(
    "/api",
    Router::new()
      .nest(
        "/auth",
        Router::new()
          .route("/login", post(handlers::auth_handler::login))
          .route("/register", post(handlers::auth_handler::register)),
      )
      .nest(
        "/users",
        Router::new()
          .route("/", get(handlers::user_handler::get_users))
          .route("/{userId}", get(handlers::user_handler::get_user_by_id)),
      )
      .route("/", get(|| async { Html("Hello World!") }))
      .fallback(handler_404) // 👈 catch-all handler
      .with_state(state),
  );
}

async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Route not found")
}
