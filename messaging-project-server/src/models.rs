use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub user_id: u64,
  pub username: String,
  pub password: String
}