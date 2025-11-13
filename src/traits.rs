use crate::types::ClientResult;

pub trait TextGenerationService {
    type Response;

    fn generate_text(
        &self,
        model: &str,
        input: &str,
    ) -> impl Future<Output = ClientResult<Self::Response>>;
}

pub trait ImageGenerationService {
    fn generate_image(&self) -> impl Future<Output = ClientResult<()>>;
}
