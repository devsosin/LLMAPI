use core::fmt;
use std::time::Duration;

use reqwest::{ClientBuilder, RequestBuilder, Response, header};
use serde::Deserialize;

#[cfg(feature = "gpt")]
use crate::AuthedGPTAPI;

#[cfg(feature = "gemini")]
use crate::AuthedGeminiAPI;

use crate::{
    LLMAPI,
    errors::ClientError,
    types::{ClientResult, ErrorResponse},
};

impl LLMAPI {
    pub fn from_env() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: ClientBuilder::new()
                .user_agent("LLM-Rust/1.0.0")
                .default_headers(headers)
                .timeout(Duration::from_secs(120))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    #[cfg(feature = "gpt")]
    pub fn authed_gpt<'a>(&'a self, token: &'a str) -> AuthedGPTAPI<'a> {
        AuthedGPTAPI { api: self, token }
    }

    #[cfg(feature = "gemini")]
    pub fn authed_gemini<'a>(&'a self, token: &'a str) -> AuthedGeminiAPI<'a> {
        AuthedGeminiAPI { api: self, token }
    }

    pub fn build_request(&self, url: &str) -> RequestBuilder {
        self.client.post(url)
    }
}

pub async fn parse_body<B: for<'de> Deserialize<'de> + fmt::Debug>(
    res: Response,
) -> ClientResult<B> {
    if !res.status().is_success() {
        let response: ErrorResponse = res.json().await.unwrap();

        return Err(ClientError::ValidationError(
            response.get_message().to_string(),
        ));
    }

    // println!("{:?}", res.text().await.unwrap());
    let res_body = res.json::<B>().await.unwrap();

    Ok(res_body)
}
