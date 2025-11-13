use reqwest::Client;

pub mod errors;
pub mod traits;
pub mod types;

#[cfg(feature = "gpt")]
pub mod gpt;

#[cfg(feature = "gpt")]
pub struct GptAPI {
    client: Client,
}

#[cfg(feature = "gemini")]
pub mod gemini;

#[cfg(feature = "gemini")]
pub struct GeminiAPI {
    client: Client,
}
