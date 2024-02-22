use warp::Filter;
use crate::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  get_post()
}

fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("users" / u64)
    .and(warp::get())
    .and_then(handlers::get_user)
}