use warp::reject;
use super::models::User;
use std::sync::Arc;
use sqlx::PgPool;

#[derive(Debug)]
struct DatabaseError;
impl reject::Reject for DatabaseError {}

pub async fn get_user(user_id: i32, pool: Arc<PgPool>) -> Result<impl warp::Reply, warp::Rejection> {
  let user_result = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id).fetch_one(&*pool).await;
  match user_result {
    Ok(user) => Ok(warp::reply::json(&user)),
    Err(_) => Err(reject::custom(DatabaseError))
  }
}