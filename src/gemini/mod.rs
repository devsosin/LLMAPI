use crate::{
    AuthedGeminiAPI,
    gemini::{
        dto::{request::GeminiRequestBody, response::GeminiResponse},
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
}
