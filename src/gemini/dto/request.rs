use serde::Serialize;

use crate::{
    gemini::types::{Content, Part},
    types::{AgentTextRequest, Thinking},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    system_instruction: Option<Content>,
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

impl From<&Thinking> for ThinkingConfig {
    fn from(t: &Thinking) -> Self {
        let thinking_level = match t {
            Thinking::High => "high",
            Thinking::Low => "low",
            Thinking::Medium => "medium",
            Thinking::Minimal => "minimal",
        }
        .into();

        ThinkingConfig { thinking_level }
    }
}

impl From<&AgentTextRequest> for GenerateContentRequest {
    fn from(req: &AgentTextRequest) -> Self {
        Self {
            system_instruction: Some(Content::new(vec![Part::new(&req.instruction)], None)),
            contents: vec![Content::new(vec![Part::new(&req.input)], None)],
            generation_config: Some(GenerationConfig {
                thinking_config: (&req.thinking).into(),
            }),
        }
    }
}

#[derive(Serialize)]
pub struct GeminiBatchRequestBody {
    batch: BatchBody,
}

#[derive(Serialize)]
struct BatchBody {
    display_name: String,
    input_config: InputConfig,
}

#[derive(Serialize)]
struct InputConfig {
    requests: Requests,
}

#[derive(Serialize)]
struct Requests {
    requests: Vec<BatchRequestContent>,
}

#[derive(Serialize)]
struct BatchRequestContent {
    request: GenerateContentRequest,
    metadata: Metadata,
}

#[derive(Serialize)]
struct Metadata {
    key: String,
}

impl GeminiBatchRequestBody {
    pub fn from_requests(
        display_name: &str,
        key_prefix: &str,
        requests: Vec<AgentTextRequest>,
    ) -> Self {
        Self {
            batch: BatchBody {
                display_name: display_name.into(),
                input_config: InputConfig {
                    requests: Requests {
                        requests: requests
                            .iter()
                            .enumerate()
                            .map(|(i, r)| BatchRequestContent {
                                request: r.into(),
                                metadata: Metadata {
                                    key: format!("{}_{}", key_prefix, i),
                                },
                            })
                            .collect(),
                    },
                },
            },
        }
    }
}
