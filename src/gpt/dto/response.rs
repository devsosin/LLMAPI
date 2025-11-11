use serde::Deserialize;

use crate::gpt::types::Role;

// TODO: move to types.rs
#[derive(Deserialize, Debug)]
pub struct GptResponse {
    id: String,
    object: String,
    created_at: u32,
    status: String, // completed
    background: bool,
    billing: Billing,
    model: String,
    output: Vec<Message>,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
pub struct Billing {
    payer: String,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    input_tokens: i32,
    output_tokens: i32,
    total_tokens: i32,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    id: String,
    #[serde(rename(deserialize = "type"))]
    message_type: String,
    status: Option<String>,
    role: Option<Role>,
    content: Option<Vec<Content>>,
    summary: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    #[serde(rename(deserialize = "type"))]
    content_type: String,
    text: String,
    annotations: Vec<String>,
    logprobs: Vec<f64>,
}
