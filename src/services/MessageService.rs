use serde::de::Error;
use serde_json::{json, Value};

use crate::components::Message::*;
use std::vec;

pub async fn get() -> reqwest::Result<Vec<Value>> {
    let messages = reqwest::get("localhost:8080/messages")
        .await?
        .text().await?;

    
    let json_res = json!(messages);
    reqwest::Result::Ok(json_res.as_array().unwrap().to_owned())
}
