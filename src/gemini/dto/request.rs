use serde::Serialize;

use crate::gemini::types::{Content, Part};

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

impl GeminiRequestBody {
    pub fn new(system_instruction: &str, text: &str, think_more: bool) -> Self {
        Self {
            system_instruction: Content::new(vec![Part::new(system_instruction)], None),
            contents: vec![Content::new(vec![Part::new(text)], None)],
            generation_config: if (think_more) {
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
