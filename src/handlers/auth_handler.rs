use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::app::AppState;

// use crate::dao::auth_dao;

#[derive(Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

/** Register user */
pub async fn register(
  State(state): State<AppState>,
  Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
  println!("->> {:<12} - login", "HANDLER");

  let db_name = state.db.name();
  let env = state.env;

  Json(json!({
      "action": "register",
      "email": payload.email,
      "db": db_name,
      "env": env,
  }))
}

/** Login user */
pub async fn login(
  State(state): State<AppState>,
  Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
  println!("->> {:<12} - get_user_by_id", "HANDLER");

  Json(json!({
      "action": "login",
      "email": payload.email,
      "db": state.db.name(),
  }))
}
