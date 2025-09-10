use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn register() -> impl IntoResponse {
  println!("->> {:<12} - get_users", "HANDLER");

  Json(json!({
      "users": [
          {
              "id": 1,
              "name": "John Doe",
              "email": "john@example.com"
          },
          {
              "id": 2,
              "name": "Jane Smith",
              "email": "jane@example.com"
          }
      ],
      "total": 2
  }))
}

pub async fn login() -> impl IntoResponse {
  println!("->> {:<12} - get_user_by_id", "HANDLER");

  Json(json!({
      "id": 1,
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": chrono::Utc::now().to_rfc3339()
  }))
}
