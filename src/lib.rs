use reqwest::Client;

pub mod client;
pub mod errors;
pub mod traits;
pub mod types;

#[cfg(feature = "gpt")]
pub mod gpt;

#[cfg(feature = "gpt")]
pub struct AuthedGPTAPI<'a> {
    api: &'a LLMAPI,
    token: &'a str,
}

#[cfg(feature = "gemini")]
pub mod gemini;

#[cfg(feature = "gemini")]
pub struct AuthedGeminiAPI<'a> {
    api: &'a LLMAPI,
    token: &'a str,
}

pub struct LLMAPI {
    client: Client,
}
