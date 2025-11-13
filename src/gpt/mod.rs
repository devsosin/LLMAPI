use crate::{
    GptAPI,
    gpt::dto::{request::GptRequestBody, response::GptResponse},
    traits::TextGenerationService,
    types::ClientResult,
};

pub mod client;
pub mod dto;
pub mod types;

impl TextGenerationService for GptAPI {
    async fn generate_text(&self, model: &str, input: &str) -> ClientResult<()> {
        let body = GptRequestBody::new(model, input);

        let response = self
            .send::<GptRequestBody, GptResponse>(
                "https://api.openai.com/v1/responses".to_string(),
                body,
            )
            .await?;

        println!("{:?}", response);

        Ok(())
    }
}
