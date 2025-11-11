use crate::types::ClientResult;

pub trait TextGenerationService {
    fn generate_text(&self, model: &str, input: &str) -> impl Future<Output = ClientResult<()>>;
}

pub trait ImageGenerationService {
    fn generate_image(&self) -> impl Future<Output = ClientResult<()>>;
}
