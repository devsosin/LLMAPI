use serde::Deserialize;

use crate::errors::ClientError;

pub type ClientResult<T> = Result<T, ClientError>;

pub enum LLMProvider {
    Gpt,
    Gemini,
}

impl From<String> for LLMProvider {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "gpt" => Self::Gpt,
            "gemini" => Self::Gemini,
            _ => Self::Gpt,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Thinking {
    High,
    Low,
    Medium,
    Minimal,
}

pub struct AgentTextRequest {
    pub instruction: String,
    pub input: String,
    pub thinking: Thinking,
}

impl AgentTextRequest {
    pub fn new(instruction: &str, input: &str, thinking: Thinking) -> Self {
        Self {
            instruction: instruction.into(),
            input: input.into(),
            thinking,
        }
    }
}

// TODO: String or byte like array? (Image)
#[derive(Debug)]
pub struct AgentTextResponse {
    response_id: String,
    content: String,
}

impl AgentTextResponse {
    pub fn new(response_id: &str, content: &str) -> Self {
        Self {
            response_id: response_id.into(),
            content: content.into(),
        }
    }

    pub fn get_response_id(&self) -> &str {
        &self.response_id
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Deserialize)]
pub struct ErrorDetail {
    message: String,
    // #[serde(rename(deserialize = "type"))]
    // error_type: String,
    // param: String,
    // code: String,
}

impl ErrorResponse {
    pub fn get_message(&self) -> &str {
        &self.error.message
    }
}

// Gemini Error Response
// {
//   "error": {
//     "code": 400,
//     "message": "API key not valid. Please pass a valid API key.",
//     "status": "INVALID_ARGUMENT",
//     "details": [
//       {
//         "@type": "type.googleapis.com/google.rpc.ErrorInfo",
//         "reason": "API_KEY_INVALID",
//         "domain": "googleapis.com",
//         "metadata": {
//           "service": "generativelanguage.googleapis.com"
//         }
//       },
//       {
//         "@type": "type.googleapis.com/google.rpc.LocalizedMessage",
//         "locale": "en-US",
//         "message": "API key not valid. Please pass a valid API key."
//       }
//     ]
//   }
// }
