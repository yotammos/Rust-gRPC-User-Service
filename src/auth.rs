use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;

const JWT_SECRET: &[u8] = b"secret";

#[derive(Clone, PartialEq)]
pub enum Role {
  User,
  Admin,
}

impl Role {
  pub fn from_str(role: &str) -> Role {
    match role {
      "Admin" => Role::Admin,
      _ => Role::User,
    }
  }
}

impl fmt::Display for Role {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Role::Admin => write!(f, "User"),
      Role::User => write!(f, "Admin"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub role: String,
  pub exp: usize,
}

pub fn create_token(uid: &str, role: &Role) -> Result<String, jsonwebtoken::errors::Error> {
  let expiration = Utc::now()
    .checked_add_signed(Duration::seconds(60))
    .expect("valid timestamp")
    .timestamp();

  let claims = Claims {
    sub: uid.to_owned(),
    role: role.to_string(),
    exp: expiration as usize,
  };

  let header = Header::default();
  encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
}
