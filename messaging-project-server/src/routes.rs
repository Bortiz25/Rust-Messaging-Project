use warp::{Filter, Reply, Rejection};
use crate::handlers;
use std::sync::Arc;
use sqlx::PgPool;
use serde::Deserialize;

pub fn routes(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  get_user(pool.clone())
    .or(login(pool.clone()))
    .or(create_user(pool.clone()))
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

fn with_db(pool:Arc<PgPool>) -> impl Filter<Extract = (Arc<PgPool>,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || pool.clone())
}