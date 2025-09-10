use axum::{
  Router,
  response::Html,
  routing::{get, post},
};
use swagger_ui_dist::{ApiDefinition, OpenApiSource, generate_routes};
use tokio::fs;

use crate::handlers;

pub fn initialize_app() -> Router {
  let api_def = ApiDefinition {
    uri_prefix: "/api/docs",
    api_definition: OpenApiSource::Inline("./swagger-ui.yml"),
    title: Some("My Super Duper API"),
  };

  return Router::new().merge(generate_routes(api_def)).nest(
    "/api",
    Router::new()
      .nest(
        "/auth",
        Router::new()
          .route("/login", post(handlers::auth::login))
          .route("/register", post(handlers::auth::register)),
      )
      .nest(
        "/users",
        Router::new()
          .route("/", get(handlers::user::get_users))
          .route("/{userId}", get(handlers::user::get_user_by_id)),
      )
      .route(
        "/docs.yml",
        get(|| async {
          fs::read_to_string("swagger-ui.yml")
            .await
            .unwrap_or_else(|_| "# OpenAPI spec not found".into());
        }),
      )
      .route("/", get(|| async { Html("Hello World!") })),
  );
}
