use core::fmt;
use std::{env, time::Duration};

use reqwest::{ClientBuilder, header};
use serde::{Deserialize, Serialize};

use crate::{
    LLMAPI,
    errors::ClientError,
    types::{ClientResult, ErrorResponse, LLMProvider},
};

impl LLMAPI {
    pub fn from_env() -> Self {
        let provider = env::var("LLM_PROVIDER").expect("Failed to load env variable: LLM_PROVIDER");
        let provider: LLMProvider = provider.into();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let api_key = match &provider {
            LLMProvider::Gpt => {
                let api_key = env::var("OPENAI_API_KEY")
                    .expect("Failed to load env variable: OPENAI_API_KEY");
                headers.insert(
                    header::AUTHORIZATION,
                    format!("Bearer {}", &api_key).parse().unwrap(),
                );
                api_key
            }
            LLMProvider::Gemini => {
                let api_key = env::var("GOOGLE_API_KEY")
                    .expect("Failed to load env variable: GOOGLE_API_KEY");
                headers.insert("x-goog-api-key", api_key.parse().unwrap());

                api_key
            }
        };

        Self {
            client: ClientBuilder::new()
                .user_agent("LLM-Rust/1.0.0")
                .default_headers(headers)
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
            provider,
        }
    }

    pub async fn send<T: Serialize, U: for<'a> Deserialize<'a> + fmt::Debug>(
        &self,
        url: String,
        body: T,
    ) -> ClientResult<U> {
        // provider에 따른 send
        let res = self.client.post(url).json(&body).send().await?;

        if !res.status().is_success() {
            let response: ErrorResponse = res.json().await.unwrap();

            return Err(ClientError::ValidationError(
                response.get_message().to_string(),
            ));
        }

        // println!("{:?}", res.text().await.unwrap());
        let res_body = res.json::<U>().await.unwrap();

        Ok(res_body)
    }
}
