use super::models::User;
use std::sync::Arc;
use sqlx::PgPool;
use warp::{ http::{Response, StatusCode}, reject, reply::{self, with_status}, Rejection, Reply};
use super::routes;
use serde_json::json;
use serde::Serialize;

#[derive(Serialize)]
struct EmptyJson {}


#[derive(Debug)]
struct DatabaseError;
impl reject::Reject for DatabaseError {}

pub async fn get_user(user_id: i32, pool: Arc<PgPool>) -> Result<impl Reply, Rejection> {
  let user_result = sqlx::query_as!( User, 
    "SELECT * FROM users WHERE user_id = $1", user_id
  ).fetch_one(&*pool).await;
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

#[derive(Serialize)]
pub struct Chat {
  chat_id: i32
}

pub async fn get_chats(user_id: i32, pool: Arc<PgPool>) -> Result<reply::WithStatus<reply::Json>, Rejection> {
  let get_chats_res = sqlx::query_as!( Chat, 
    "SELECT (c.chat_id)
    FROM user_to_chat u2c
    JOIN chats c
    ON c.chat_id = u2c.chat_id
    WHERE user_id=$1", user_id
  ).fetch_all(&*pool).await;

  let chats = match get_chats_res {
    Ok(c) => c,
    Err(_) => return Err(reject::custom(DatabaseError))
  };
  
  Ok(reply::with_status(reply::json(&chats), StatusCode::OK))
}


pub async fn create_chat(user_id: i32, body: routes::CreateChatRequestBody, pool: Arc<PgPool>) -> Result<reply::WithStatus<impl Reply>, Rejection> {
  let buddy_id = body.buddy_id;
  
  // validate user and buddy exist in db
  let get_user_res = sqlx::query_as!( User, 
    "SELECT * FROM users WHERE user_id = $1", user_id
  ).fetch_one(&*pool).await;

  let user = match get_user_res {
    Ok(u) => u,
    Err(_) => return Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
  };

  let get_buddy_res = sqlx::query_as!( User, 
    "SELECT * FROM users WHERE user_id = $1", buddy_id
  ).fetch_one(&*pool).await;

  let buddy = match get_buddy_res {
    Ok(u) => u,
    Err(_) => return Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
  };

  // check if chat already exists -- checks that there the two users have one shared chat, which won't work if we ever implement group chats
  let existing_chat_res = sqlx::query!(
    "SELECT COUNT(DISTINCT c.chat_id) AS shared_chat_count
     FROM chats c
     JOIN user_to_chat uc1 ON c.chat_id = uc1.chat_id AND uc1.user_id = $1
     JOIN user_to_chat uc2 ON c.chat_id = uc2.chat_id AND uc2.user_id = $2
   ", user.user_id, buddy.user_id).fetch_one(&*pool).await;
   let existing_chat_count = match existing_chat_res {
    Ok(res) => res.shared_chat_count.unwrap_or(0),
    Err(_) => return Err(reject::custom(DatabaseError))
   };

   if existing_chat_count > 0 {
    return Ok(reply::with_status("Already Exists", StatusCode::CONFLICT));
   };

  
  // create new chat
  let chat_create_res = sqlx::query_as!(Chat, "INSERT INTO chats DEFAULT VALUES RETURNING chat_id").fetch_one(&*pool).await;
  let chat_id = match chat_create_res {
    Ok(res) => res.chat_id,
    Err(_) => return Err(reject::custom(DatabaseError))
  };

  // connect users to new chat
  let user_connection_res = sqlx::query!("INSERT INTO user_to_chat (user_id, chat_id) VALUES ($1, $2)", user.user_id, chat_id).execute(&*pool).await;
  if let Err(_) = user_connection_res {
    return Err(reject::custom(DatabaseError));
  };

  let buddy_connection_res = sqlx::query!("INSERT INTO user_to_chat (user_id, chat_id) VALUES ($1, $2)", buddy.user_id, chat_id).execute(&*pool).await;
  if let Err(_) = buddy_connection_res {
    return Err(reject::custom(DatabaseError));
  };

  Ok(reply::with_status("Created", StatusCode::CREATED))
}

pub async fn create_message(chat_id: i32, user_id: i32, body: routes::CreateMessageRequestBody, pool: Arc<PgPool>) -> Result<reply::WithStatus<impl Reply>, Rejection> {
  let message = body.message;
  let chat_exists_res = sqlx::query!("SELECT EXISTS(SELECT (chat_id) FROM chats WHERE chat_id=$1) as exists", chat_id).fetch_one(&*pool).await;
  let exists = match chat_exists_res {
    Ok(res) => res.exists.unwrap_or(false),
    Err(_) => return Err(reject::custom(DatabaseError))
  };
  if !exists {
    return Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
  }

  let create_res = sqlx::query!(
    "INSERT INTO messages (chat_id, sent_from, message) VALUES ($1, $2, $3)", 
    chat_id, user_id, message
  ).execute(&*pool).await;
  if let Err(e) = create_res {
    println!("{}", e);
    return Err(reject::custom(DatabaseError));
  }
  Ok(reply::with_status("Created", StatusCode::CREATED))
}
#[derive(Serialize)]
pub struct Message {
  message_id: i32,
  chat_id: i32,
  sent_from: i32,
  message: String
}

pub async fn get_messages(chat_id: i32, _user_id: i32, pool: Arc<PgPool>) -> Result<reply::WithStatus<reply::Json>, Rejection> {
  let chat_exists_res = sqlx::query!("SELECT EXISTS(SELECT (chat_id) FROM chats WHERE chat_id=$1) as exists", chat_id).fetch_one(&*pool).await;
  let exists = match chat_exists_res {
    Ok(res) => res.exists.unwrap_or(false),
    Err(_) => return Err(reject::custom(DatabaseError))
  };
  if !exists {
    return Ok(reply::with_status(reply::json(&EmptyJson{}), StatusCode::NOT_FOUND))
  }

  let messages_res = sqlx::query_as!(Message, "SELECT * FROM messages WHERE chat_id = $1", chat_id).fetch_all(&*pool).await;
  let messages = match messages_res {
    Ok(m) => m,
    Err(_) => return Err(reject::custom(DatabaseError))
  };

  Ok(with_status(reply::json(&messages), StatusCode::OK))
}