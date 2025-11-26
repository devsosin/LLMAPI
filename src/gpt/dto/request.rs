use serde::Serialize;

use crate::gpt::models::GptModel;

#[derive(Serialize)]
pub struct GptRequestBody {
    model: GptModel,
    reasoning: Option<ReasoningConfig>,
    instructions: String,
    input: String,
}

#[derive(Serialize)]
struct ReasoningConfig {
    effort: String,
}

impl GptRequestBody {
    pub fn new(model: &str, instructions: &str, input: &str, reasoning: bool) -> Self {
        Self {
            model: model.into(),
            reasoning: if reasoning {
                None
            } else {
                Some(ReasoningConfig {
                    effort: "low".into(),
                })
            },
            instructions: instructions.into(),
            input: input.into(),
        }
    }
}
