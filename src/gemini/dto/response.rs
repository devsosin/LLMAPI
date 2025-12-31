use serde::Deserialize;

use crate::{
    gemini::{models::GeminiModel, types::Content},
    types::AgentTextResponse,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
    model_version: GeminiModel,
    response_id: String,
}

impl Into<AgentTextResponse> for GeminiResponse {
    fn into(self) -> AgentTextResponse {
        AgentTextResponse::new(
            &self.response_id,
            self.candidates.last().unwrap().content.get_text(),
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    content: Content,
    // finish_reason: Option<String>,
    // index: i32
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeminiBatchResponse {
    name: String,
    done: Option<bool>,

    metadata: Option<BatchMetadata>,
    response: Option<BatchResponse>,
}

#[derive(Deserialize, Debug)]
pub struct BatchMetadata {
    state: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchResponse {
    inlined_responses: InlinedResponse,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InlinedResponse {
    inlined_responses: Vec<InlineResponse>,
}

#[derive(Deserialize, Debug)]
pub struct InlineResponse {
    response: ResponseCandidate,
    metadata: KeyMetadata,
}

#[derive(Deserialize, Debug)]
pub struct ResponseCandidate {
    candidates: Vec<Candidate>,
    response_id: String,
}

#[derive(Deserialize, Debug)]
pub struct KeyMetadata {
    key: String,
}

impl GeminiBatchResponse {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_done(&self) -> &Option<bool> {
        &self.done
    }

    pub fn get_responses(&self) -> Vec<AgentTextResponse> {
        self.response
            .as_ref()
            .unwrap()
            .inlined_responses
            .inlined_responses
            .iter()
            .map(|r| {
                let response_id = &r.response.response_id;
                let content = r.response.candidates.last().unwrap().content.get_text();

                AgentTextResponse::new(response_id, content.into())
            })
            .collect()
    }
}
