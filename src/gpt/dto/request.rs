use serde::Serialize;

use crate::gpt::models::GptModel;

#[derive(Serialize)]
pub struct GptRequestBody {
    model: GptModel,
    input: String,
}

impl GptRequestBody {
    pub fn new(model: &str, input: &str) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
        }
    }
}
