use crate::{
    LLMAPI,
    gpt::dto::{request::GptRequestBody, response::GptResponse},
    traits::TextGenerationService,
    types::ClientResult,
};

pub mod dto;
pub mod types;

// TODO?: Split LLMAPI to GptAPI, GeminiAPI -> dependencies with features (gpt, gemini)
impl TextGenerationService for LLMAPI {
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
