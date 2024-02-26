use warp::Filter;
use crate::handlers;
use std::sync::Arc;
use sqlx::PgPool;

pub fn routes(pool: Arc<PgPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  get_post(pool)
}

fn get_post(pool: Arc<PgPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("users" / i32)
    .and(warp::get())
    .and(with_db(pool))
    .and_then(handlers::get_user)
}

fn with_db(pool:Arc<PgPool>) -> impl Filter<Extract = (Arc<PgPool>,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || pool.clone())
}