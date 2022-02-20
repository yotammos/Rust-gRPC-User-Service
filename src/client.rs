use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use user_service::user_service_client::UserServiceClient;
use user_service::{AuthenticateRequest, LoginRequest};
mod auth;

pub mod user_service {
  tonic::include_proto!("user_service");
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginSession {
  user_id: String,
  email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

  let request = tonic::Request::new(LoginRequest {
    email: String::from("abc@123.com"),
    password: String::from("edf"),
  });
  // let request = tonic::Request::new(ListUsersRequest {});
  let response = client.login(request).await?;

  println!("RESPONSE={:?}", response);
  let token = response.into_inner().token;
  let user_id = decode::<auth::Claims>(
    &token,
    &DecodingKey::from_secret("secret".as_ref()),
    &Validation::default(),
  );
  println!("Decoded token = {:?}", user_id);

  let response = client
    .authenticate(tonic::Request::new(AuthenticateRequest { token }))
    .await?;
  let inner = response.into_inner();
  println!("User ID = {:?}", inner.user_id);
  println!("Role = {:?}", inner.role);
  Ok(())
}
