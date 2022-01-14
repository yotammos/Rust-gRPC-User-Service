use user_service::user_service_client::UserServiceClient;
use user_service::LoginRequest;

pub mod user_service {
  tonic::include_proto!("user_service");
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
  Ok(())
}
