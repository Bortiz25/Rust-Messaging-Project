use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct User {
  pub user_id: i32,
  pub username: String,
  pub password: String
}