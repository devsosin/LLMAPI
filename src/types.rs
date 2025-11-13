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
