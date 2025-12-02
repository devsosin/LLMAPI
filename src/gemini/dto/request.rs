use serde::Serialize;

use crate::{
    gemini::types::{Content, Part},
    types::AgentTextRequest,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiRequestBody {
    system_instruction: Content,
    contents: Vec<Content>,
    generation_config: Option<GenerationConfig>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    thinking_config: ThinkingConfig,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThinkingConfig {
    thinking_level: String,
}

impl From<AgentTextRequest> for GeminiRequestBody {
    fn from(req: AgentTextRequest) -> Self {
        Self {
            system_instruction: Content::new(vec![Part::new(&req.instruction)], None),
            contents: vec![Content::new(vec![Part::new(&req.input)], None)],
            generation_config: if (req.think_more) {
                Some(GenerationConfig {
                    thinking_config: ThinkingConfig {
                        thinking_level: "low".into(),
                    },
                })
            } else {
                None
            },
        }
    }
}
