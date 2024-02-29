use super::models::User;
use std::sync::Arc;
use sqlx::PgPool;
use warp::{Filter, Reply, Rejection, reply, reject, http::{StatusCode, Response}};
use super::routes;
use serde_json::json;


#[derive(Debug)]
struct DatabaseError;
impl reject::Reject for DatabaseError {}

pub async fn get_user(user_id: i32, pool: Arc<PgPool>) -> Result<impl Reply, Rejection> {
  let user_result = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id).fetch_one(&*pool).await;
  match user_result {
    Ok(user) => Ok(reply::json(&user)),
    Err(_) => Err(reject::custom(DatabaseError))
  }
}


pub struct LoginResponseBody {
  token: String,
}
impl Reply for LoginResponseBody {
  fn into_response(self) -> reply::Response {
    let body = json!({
      "token": self.token,
    }).to_string();

    Response::builder()
      .header("Content-Type", "application/json")
      .body(warp::hyper::Body::from(body))
      .unwrap()
  }
}

pub async fn login(body: routes::LoginRequestBody, pool: Arc<PgPool>) -> Result<reply::WithStatus<LoginResponseBody>, Rejection> {
  let username: String = body.username;
  let password: String = body.password;
  let response_body = LoginResponseBody {token: String::from("Not Implemented")};
  Ok(reply::with_status(response_body, StatusCode::CREATED))
}

pub async fn create_user(body: routes::LoginRequestBody, pool: Arc<PgPool>) -> Result<reply::WithStatus<impl Reply>, Rejection> {
  let username: String = body.username;
  let password: String = body.password;
  Ok(reply::with_status("Not implemented", StatusCode::CREATED))
}