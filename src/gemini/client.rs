use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{AuthedGeminiAPI, client::parse_body, types::ClientResult};

impl<'a> AuthedGeminiAPI<'a> {
    pub async fn send<T: Serialize, U: for<'de> Deserialize<'de> + fmt::Debug>(
        &self,
        url: String,
        body: T,
    ) -> ClientResult<U> {
        let res = self
            .api
            .build_request(&url)
            .header("x-goog-api-key", self.token)
            .json(&body)
            .send()
            .await?;

        parse_body(res).await
    }
}
