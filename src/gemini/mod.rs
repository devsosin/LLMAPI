mod client;
mod dto;
pub mod models;
pub(crate) mod types;

use std::time::Duration;

use tokio::time::sleep;

use crate::{
    AuthedGeminiAPI,
    errors::ClientError,
    gemini::{
        dto::{
            request::{GeminiBatchRequestBody, GenerateContentRequest},
            response::{GeminiBatchResponse, GeminiResponse},
        },
        models::GeminiModel,
    },
    traits::{ModelSelection, TextGenerationService},
    types::{AgentTextRequest, AgentTextResponse, ClientResult},
};

impl<'a> ModelSelection for AuthedGeminiAPI<'a> {
    type Model = GeminiModel;

    fn get_model_str(&self, model: Self::Model) -> String {
        model.to_string()
    }
}

impl<'a> TextGenerationService for AuthedGeminiAPI<'a> {
    async fn generate_text(
        &self,
        model: GeminiModel,
        request: AgentTextRequest,
    ) -> ClientResult<AgentTextResponse> {
        if !model.check_thinking(&request.thinking) {
            return Err(ClientError::ValidationError("Thinking Config".into()));
        };

        let body = (&request).into();

        let response = self
            .send::<GenerateContentRequest, GeminiResponse>(
                format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                    model.to_string()
                ),
                body,
            )
            .await?;

        // println!("{:?}", response);

        Ok(response.into())
    }

    async fn batch_generate_text(
        &self,
        model: Self::Model,
        display_name: &str,
        key_prefix: &str,
        requests: Vec<AgentTextRequest>,
    ) -> ClientResult<Vec<AgentTextResponse>> {
        if !model.check_thinking(&requests[0].thinking) {
            return Err(ClientError::ValidationError("Thinking Config".into()));
        };

        let body = GeminiBatchRequestBody::from_requests(display_name, key_prefix, requests.into());

        let response = self
            .send::<GeminiBatchRequestBody, GeminiBatchResponse>(
                format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/{}:batchGenerateContent",
                    model.to_string()
                ),
                body,
            )
            .await?;

        let batch_name = response.get_name();
        // println!("Batch Name: {}", batch_name);

        loop {
            if let Ok(r) = self
                .send_get::<GeminiBatchResponse>(format!(
                    "https://generativelanguage.googleapis.com/v1beta/{}",
                    batch_name,
                ))
                .await
            {
                if let Some(done) = r.get_done() {
                    if *done {
                        // println!("Gemini Batch Response: {:?}", r);
                        return Ok(r.get_responses());
                    }
                }
            }

            sleep(Duration::from_secs(10)).await;
            continue;
        }
    }
}
