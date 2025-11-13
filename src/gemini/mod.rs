use crate::{
    GeminiAPI,
    gemini::{
        dto::{request::GeminiRequestBody, response::GeminiResponse},
        models::GeminiModel,
    },
    traits::TextGenerationService,
    types::ClientResult,
};

pub mod client;
pub mod dto;
pub mod models;
pub mod types;

impl TextGenerationService for GeminiAPI {
    type Response = GeminiResponse;

    async fn generate_text(&self, model: &str, input: &str) -> ClientResult<Self::Response> {
        let body = GeminiRequestBody::new(input);
        let model: GeminiModel = model.into();

        let response = self
            .send::<GeminiRequestBody, GeminiResponse>(
                format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                    model.as_str()
                ),
                body,
            )
            .await?;

        println!("{:?}", response);

        Ok(response)
    }
}
