extern crate serde_json;
extern crate tokio;
use serde_json::{Value, Map, Number};
use std::time::{SystemTime, UNIX_EPOCH, Duration};


#[tokio::main]
async fn main() {
    let secs;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => secs = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
   
   let mut json_body = Map::new();
    json_body.insert("entity".to_string(), Value::String("/users/toby/goyum/src/main.rs".to_string()));
    json_body.insert("type".to_string(), Value::String("file".to_string()));
    json_body.insert("time".to_string(), Value::Number(Number::from(secs)));
    json_body.insert("project".to_string(), Value::String("wakatime_api_lier".to_string()));
    json_body.insert("langauge".to_string(), Value::String("Rust".to_string()));
    json_body.insert("lines".to_string(), Value::Number(Number::from(500)));

    let client =  reqwest::Client::new();
    let res = client.post("https://wakatime.com/api/v1/users/current/heartbeats")
    .json(&json_body).send().await;
    
    println!("{:#?}", res);

}
