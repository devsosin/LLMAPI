use core::fmt;
use std::{env, time::Duration};

use reqwest::{ClientBuilder, header};
use serde::{Deserialize, Serialize};

use crate::{
    GptAPI,
    errors::ClientError,
    types::{ClientResult, ErrorResponse},
};

impl GptAPI {
    pub fn from_env() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        let api_key =
            env::var("OPENAI_API_KEY").expect("Failed to load env variable: OPENAI_API_KEY");
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", &api_key).parse().unwrap(),
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

    pub async fn send<T: Serialize, U: for<'a> Deserialize<'a> + fmt::Debug>(
        &self,
        url: String,
        body: T,
    ) -> ClientResult<U> {
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
