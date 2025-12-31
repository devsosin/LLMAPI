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
    // Option String, enum?
    thinking_level: String,
}

// Thinking Level - low, high(default)
//              only Flash => medium, minimal(=no thinking)

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

// GeminiBatchRequestBody { batch : {display_name, input_config: requests: {requests: [{request:{contents}, metadata: {key}}]}}}
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
    request: RequestContent,
    metadata: Metadata,
}

#[derive(Serialize)]
struct RequestContent {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Metadata {
    key: String,
}

impl GeminiBatchRequestBody {
    pub fn from_requests(display_name: &str, requests: Vec<AgentTextRequest>) -> Self {
        Self {
            batch: BatchBody {
                display_name: display_name.into(),
                input_config: InputConfig {
                    requests: Requests {
                        requests: requests
                            .iter()
                            .map(|r| BatchRequestContent {
                                request: RequestContent {
                                    contents: vec![Content::new(vec![Part::new(&r.input)], None)],
                                },
                                metadata: Metadata { key: "()".into() },
                            })
                            .collect(),
                    },
                },
            },
        }
    }
}
