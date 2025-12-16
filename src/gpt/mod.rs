use crate::{
    AuthedGPTAPI,
    gpt::{
        dto::{request::GptRequestBody, response::GptResponse},
        models::GptModel,
    },
    traits::{ModelSelection, TextGenerationService},
    types::{AgentTextRequest, AgentTextResponse, ClientResult},
};

mod client;
mod dto;
pub mod models;
pub mod types;

impl<'a> ModelSelection for AuthedGPTAPI<'a> {
    type Model = GptModel;

    fn get_model_str(&self, model: Self::Model) -> String {
        model.to_string()
    }
}

impl<'a> TextGenerationService for AuthedGPTAPI<'a> {
    async fn generate_text(
        &self,
        model: Self::Model,
        request: AgentTextRequest,
    ) -> ClientResult<AgentTextResponse> {
        let body = GptRequestBody::new(
            model,
            &request.instruction,
            &request.input,
            request.think_more,
        );

        let response = self
            .send::<GptRequestBody, GptResponse>(
                "https://api.openai.com/v1/responses".to_string(),
                body,
            )
            .await?;

        // println!("{:?}", response);

        Ok(response.into())
    }
}
