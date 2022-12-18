use std::collections::HashMap;
use gql_client::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    getCursor: String,
}

#[derive(Deserialize)]
pub struct UserData {
    someUsers : Vec<User>
}

#[derive(Deserialize)]
pub struct User {
    id: i32,
    name : String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:3030/graphql";
    let mut headers = HashMap::new();
    headers.insert("authorization", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6Im1hdHlhc2guZGFuaWxAbWFpbC5ydSIsInBhc3N3b3JkIjoiMjFhYzVhM2JlYzUxYjU2MjM0NDMwZDhkMDg3NmFkZjgiLCJleHAiOjE2NzE0MzYxNTl9.VKdZEerASqdmN_F2Uay5xl-9vhjGz6RelzmF_ouvCCJQaFeQBit5f7MW037f-KmmLb2OOKV3C1RXhVSH6dRBlw");
    let create_cursor = r#"
       query {
        getCursor
        }
   "#;

    let client = Client::new_with_headers(endpoint, headers);
    let data = client.query::<Data>(create_cursor).await.unwrap();
    let cursor = data.unwrap().getCursor;

    let get_users = format!("query {{someUsers(action :\"forward\", count : 1, cursor : \"{cursor}\") {{id, name}}}}",cursor = cursor );

    for i in 0..10 {
        let data = client.query::<UserData>(&*get_users.clone()).await.unwrap().unwrap();
        println!("id : {}\nname : {}\n", data.someUsers[0].id.clone(), data.someUsers[0].name.clone())
    }

    Ok(())
}