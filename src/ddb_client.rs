pub mod ddb_client {
  use crate::user_service::User;
  use aws_sdk_dynamodb::error::{PutItemError, ScanError};
  use aws_sdk_dynamodb::model::AttributeValue;
  use aws_sdk_dynamodb::output::PutItemOutput;
  use aws_sdk_dynamodb::{Client, Region, SdkError};
  use std::collections::HashMap;
  use std::str::FromStr;

  fn unwrap_number_attribute<T: FromStr>(
    atr: Option<&AttributeValue>,
  ) -> Result<T, <T as FromStr>::Err> {
    atr.unwrap().as_n().unwrap().parse::<T>()
  }

  fn item_to_user(item: &HashMap<String, AttributeValue>) -> User {
    let id = item.get("id").unwrap().as_s().unwrap().to_string();
    let email = item.get("email").unwrap().as_s().unwrap().to_string();
    let password = item.get("password").unwrap().as_s().unwrap().to_string();
    let created_at = unwrap_number_attribute(item.get("created_at")).unwrap();
    User {
      id,
      email,
      password,
      created_at,
    }
  }

  pub async fn list_users() -> Result<Vec<User>, SdkError<ScanError>> {
    let shared_config = aws_config::from_env()
      .region(Region::new("us-east-1"))
      .load()
      .await;
    let client = Client::new(&shared_config);
    let request = client.scan().table_name("users");
    request.send().await.map(|output| {
      output
        .items
        .unwrap()
        .iter()
        .map(|item| item_to_user(item))
        .collect::<Vec<User>>()
    })
  }

  pub async fn create_user(
    id: &String,
    email: String,
    password: String,
    created_at: u128,
  ) -> Result<PutItemOutput, SdkError<PutItemError>> {
    let shared_config = aws_config::from_env()
      .region(Region::new("us-east-1"))
      .load()
      .await;
    let client = Client::new(&shared_config);
    let request = client
      .put_item()
      .table_name("users")
      .item("id", AttributeValue::S(id.to_string()))
      .item("email", AttributeValue::S(email))
      .item("password", AttributeValue::S(password))
      .item("created_at", AttributeValue::N(created_at.to_string()));
    request.send().await
  }
}
