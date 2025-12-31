use std::time::Duration;

use tokio::time::sleep;

use crate::{
    AuthedGeminiAPI,
    gemini::{
        dto::{
            request::{GeminiBatchRequestBody, GeminiRequestBody},
            response::{GeminiBatchResponse, GeminiResponse},
        },
        models::GeminiModel,
    },
    traits::{ModelSelection, TextGenerationService},
    types::{AgentTextRequest, AgentTextResponse, ClientResult},
};

mod client;
mod dto;
pub mod models;
pub mod types;

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
        let body = request.into();

        let response = self
            .send::<GeminiRequestBody, GeminiResponse>(
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
        requests: Vec<AgentTextRequest>,
    ) -> ClientResult<Vec<AgentTextResponse>> {
        // display_name 설정
        // 각 metadata key 설정 -> 결과 매칭용..? -> 순서대로 나오긴 하는듯..한데
        let body = GeminiBatchRequestBody::from_requests("test", requests.into());

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
        // 생각보다 좀 오래걸림.. 가격은 싸니까 괜찮..나?
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
                        println!("Gemini Batch Response: {:?}", r);
                        return Ok(r.get_responses());
                    }
                }
            }

            sleep(Duration::from_secs(10)).await;
            continue;
        }
    }
}
