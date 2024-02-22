use warp::Filter;
use super::models::User;

pub async fn get_user(user_id: u64) -> Result<impl warp::Reply, warp::Rejection> {
  let user = User {
    user_id,
    username: String::from("lmartin9"),
    password: String::from("password"),
  };
  Ok(warp::reply::json(&user))
}