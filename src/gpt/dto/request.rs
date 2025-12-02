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
    pub fn new(model: GptModel, instructions: &str, input: &str, reasoning: bool) -> Self {
        Self {
            reasoning: if model.is_reasoning() {
                Some(ReasoningConfig {
                    effort: match reasoning {
                        true => "high",
                        false => "low",
                    }
                    .into(),
                })
            } else {
                None
            },
            instructions: instructions.into(),
            input: input.into(),
            model,
        }
    }
}
