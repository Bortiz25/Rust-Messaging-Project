use crate::{auth::authorize, handlers::{self}, models::User};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::{http::{header::{HeaderMap, HeaderValue}, StatusCode}, reject, reply::{self, with_status}, Filter, Rejection, Reply};

#[derive(Debug)]
struct JwtError;
impl reject::Reject for JwtError {}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
  user_id: i32,
  username: String
}

pub fn routes(pool: Arc<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_user(pool.clone())
        .or(login(pool.clone()))
        .or(create_user(pool.clone()))
        .or(create_chat(pool.clone()))
        .or(create_message(pool.clone()))
        .or(get_chats(pool.clone()))
        .or(get_messages(pool.clone()))
        .or(get_user_with_token(pool.clone()))
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
    pub password: String,
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
    pub buddy_id: String,
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
    pub message: String,
}

fn create_message(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("chats" / String)
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
    warp::path!("chats" / String / "messages")
        .and(warp::get())
        .and(with_auth())
        .and(with_db(pool))
        .and_then(handlers::get_messages)
}

fn get_user_with_token(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(with_db(pool))
        .and_then(handlers::get_user_with_token)
}

fn with_db(
    pool: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn with_auth(
    pool: Arc<PgPool>, headers: &HeaderMap<HeaderValue>
) -> impl Filter<Extract = (i32,), Error = std::convert::Infallible> + Clone {
    let user_id_res = authorize(headers.clone());
    let user_id = match user_id_res {
      Ok(suid) => {
        let ouid = suid.parse::<i32>();
        match ouid {
          Ok(uid) => uid,
          Err(_) => panic!()
        }
      },
      Err(_) => panic!()
    };
    println!("{}", user_id);
    let get_user_res = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id=$1", user_id).fetch_optional(&*pool).await;
    let user = match get_user_res {
      Ok(ouser) => match ouser {
        Some(user) => UserResponse{ user_id: user.user_id, username: user.username },
        None => panic!("Ok(reply::with_status(reply::json(&EmptyJson'{{''}}'), StatusCode::NOT_FOUND))")
      }
      Err(_) => panic!("Err(reject::custom(DatabaseError))")
    };
    warp::any().map(move || user_id)
}
