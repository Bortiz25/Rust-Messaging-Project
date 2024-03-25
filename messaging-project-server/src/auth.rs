use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use warp::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};

const BEARER: &str = "Bearer ";
// This should be in the .env file instead
const JWT_SECRET: &[u8] = b"secret";

#[derive(Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: i64,
}

pub fn create_jwt(uid: i32) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::try_weeks(52).unwrap())
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        id: uid.to_string(),
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
}

pub enum AuthorizeError {
    NoAuthHeaderError,
    InvalidAuthHeaderError,
    DecodeError,
}

pub fn authorize(headers: HeaderMap<HeaderValue>) -> Result<String, AuthorizeError> {
    let token_data = match jwt_from_header(&headers) {
        Ok(jwt) => {
            println!("{}", jwt);
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            );
            match decoded {
                Ok(token_data) => token_data,
                Err(e) => {
                    println!("{}", e);
                    return Err(AuthorizeError::DecodeError);
                }
            }
        }
        Err(_) => {
            println!("0");
            return Err(AuthorizeError::DecodeError);
        }
    };

    Ok(token_data.claims.id)
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AuthorizeError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthorizeError::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthorizeError::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(AuthorizeError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
