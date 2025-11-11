use reqwest::Client;

use crate::types::LLMProvider;

pub mod client;
pub mod errors;
pub mod traits;
pub mod types;

// llm services
pub mod gemini;
pub mod gpt;

pub struct LLMAPI {
    client: Client,
    provider: LLMProvider,
}
