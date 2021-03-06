mod auth;
mod ddb_client;

use auth::{create_token, Role};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tonic::{transport::Server, Request, Response, Status};
use user_service::user_service_server::{UserService, UserServiceServer};
use user_service::{
  AuthenticateRequest, AuthenticateResponse, CreateUserRequest, CreateUserResponse,
  ListUsersRequest, ListUsersResponse, LoginRequest, LoginResponse, User,
};
use uuid::Uuid;

pub mod user_service {
  tonic::include_proto!("user_service");
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginSession {
  user_id: String,
  email: String,
}

#[derive(Debug, Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
  async fn create_user(
    &self,
    req: Request<CreateUserRequest>,
  ) -> Result<Response<CreateUserResponse>, Status> {
    let id = Uuid::new_v4().to_string();
    let inner_req = req.into_inner();
    let created_at = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();
    match ddb_client::ddb_client::create_user(
      &id,
      inner_req.email,
      inner_req.password,
      created_at.as_millis(),
    )
    .await
    {
      Ok(_result) => Ok(Response::new(CreateUserResponse { id })),
      Err(error) => Err(Status::internal(error.to_string())),
    }
  }

  async fn list_users(
    &self,
    _req: Request<ListUsersRequest>,
  ) -> Result<Response<ListUsersResponse>, Status> {
    match ddb_client::ddb_client::list_users().await {
      Ok(users) => Ok(Response::new(ListUsersResponse { users })),
      Err(error) => Err(Status::internal(error.to_string())),
    }
  }

  async fn login(&self, req: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
    let request = req.into_inner();
    match ddb_client::ddb_client::list_users().await {
      Ok(users) => {
        let unique_users: Vec<&User> = users
          .iter()
          .filter(|user| user.email == request.email)
          .collect::<Vec<&User>>();
        if unique_users.is_empty() {
          Err(Status::not_found("user not found"))
        } else {
          let user = unique_users.first().unwrap();
          if user.password == request.password {
            let token = create_token(&user.id, &Role::Admin).unwrap();
            Ok(Response::new(LoginResponse { token }))
          } else {
            Err(Status::not_found("user not found"))
          }
        }
      }
      Err(error) => Err(Status::internal(error.to_string())),
    }
  }

  async fn authenticate(
    &self,
    req: Request<AuthenticateRequest>,
  ) -> Result<Response<AuthenticateResponse>, Status> {
    let token = req.into_inner().token;
    let decoded = decode::<auth::Claims>(
      &token,
      &DecodingKey::from_secret("secret".as_ref()),
      &Validation::default(),
    );
    match decoded {
      Ok(result) => {
        let claims: auth::Claims = result.claims;
        Ok(Response::new(AuthenticateResponse {
          user_id: claims.sub,
          role: claims.role,
        }))
      }
      Err(error) => Err(Status::internal(error.to_string())),
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "[::1]:50051".parse()?;
  let user_service = UserServiceImpl::default();

  Server::builder()
    .add_service(UserServiceServer::new(user_service))
    .serve(address)
    .await?;

  Ok(())
}
