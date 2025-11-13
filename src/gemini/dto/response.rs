use serde::Deserialize;

use crate::gemini::{models::GeminiModel, types::Content};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
    model_version: GeminiModel,
    response_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    content: Content,
    // finish_reason: Option<String>,
    // index: i32
}

impl GeminiResponse {
    pub fn get_content(&self) -> &str {
        self.candidates.last().unwrap().content.get_text()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
    prompt_tokens_details: Vec<TokenDetail>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetail {
    modality: String,
    token_count: i32,
}
