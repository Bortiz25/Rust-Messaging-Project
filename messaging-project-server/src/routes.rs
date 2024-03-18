use warp::{Filter, Reply, Rejection};
use crate::handlers;
use std::sync::Arc;
use sqlx::PgPool;
use serde::Deserialize;

pub fn routes(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  get_user(pool.clone())
    .or(login(pool.clone()))
    .or(create_user(pool.clone()))
    .or(create_chat(pool.clone()))
    .or(create_message(pool.clone()))
    .or(get_chats(pool.clone()))
    .or(get_messages(pool.clone()))
}

fn get_user(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("users" / i32)
    .and(warp::get())
    .and(with_db(pool))
    .and_then(handlers::get_user)
}

#[derive(Deserialize)]
pub struct LoginRequestBody {
  pub username: String,
  pub password: String
}

fn login(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("login")
    .and(warp::post())
    .and(warp::body::json::<LoginRequestBody>())
    .and(with_db(pool))
    .and_then(handlers::login)
}

fn create_user(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("users")
    .and(warp::post())
    .and(warp::body::json::<LoginRequestBody>())
    .and(with_db(pool))
    .and_then(handlers::create_user)
}

#[derive(Deserialize)]
pub struct CreateChatRequestBody {
  pub buddy_id: i32
}

fn create_chat(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("chats")
    .and(warp::post())
    .and(with_auth())
    .and(warp::body::json::<CreateChatRequestBody>())
    .and(with_db(pool))
    .and_then(handlers::create_chat)
}

#[derive(Deserialize)]
pub struct CreateMessageRequestBody {
  pub message: String
}

fn create_message(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("chats" / i32)
    .and(warp::post())
    .and(with_auth())
    .and(warp::body::json::<CreateMessageRequestBody>())
    .and(with_db(pool))
    .and_then(handlers::create_message)
}

fn get_chats(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("chats")
    .and(warp::get())
    .and(with_auth())
    .and(with_db(pool))
    .and_then(handlers::get_chats)
}

fn get_messages(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("chats" / i32 / "messages")
    .and(warp::get())
    .and(with_auth())
    .and(with_db(pool))
    .and_then(handlers::get_messages)
}

fn with_db(pool:Arc<PgPool>) -> impl Filter<Extract = (Arc<PgPool>,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || pool.clone())
}

fn with_auth() -> impl Filter<Extract = (i32,), Error = std::convert::Infallible> + Clone {
  let user_id = 1;
  warp::any().map(move || user_id)
}