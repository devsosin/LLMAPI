use crate::types::{AgentTextRequest, AgentTextResponse, ClientResult};

// TODO? is needed?
pub trait ModelSelection {
    type Model;

    fn get_model_str(&self, model: Self::Model) -> String;
}

pub trait TextGenerationService: ModelSelection {
    fn generate_text(
        &self,
        model: Self::Model,
        request: AgentTextRequest,
    ) -> impl Future<Output = ClientResult<AgentTextResponse>>;

    fn batch_generate_text(
        &self,
        model: Self::Model,
        display_name: &str,
        key_prefix: &str,
        requests: Vec<AgentTextRequest>,
    ) -> impl Future<Output = ClientResult<Vec<AgentTextResponse>>>;
}

pub trait ImageGenerationService: ModelSelection {
    fn generate_image(&self) -> impl Future<Output = ClientResult<()>>;
}
