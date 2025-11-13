use crate::{
    GptAPI,
    gpt::dto::{request::GptRequestBody, response::GptResponse},
    traits::TextGenerationService,
    types::ClientResult,
};

pub mod client;
pub mod dto;
pub mod models;
pub mod types;

impl TextGenerationService for GptAPI {
    type Response = GptResponse;
    async fn generate_text(&self, model: &str, input: &str) -> ClientResult<Self::Response> {
        let body = GptRequestBody::new(model, input);

        let response = self
            .send::<GptRequestBody, GptResponse>(
                "https://api.openai.com/v1/responses".to_string(),
                body,
            )
            .await?;

        println!("{:?}", response);

        Ok(response)
    }
}
