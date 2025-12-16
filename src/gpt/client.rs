use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{AuthedGPTAPI, client::parse_body, types::ClientResult};

impl<'a> AuthedGPTAPI<'a> {
    pub async fn send<T: Serialize, U: for<'de> Deserialize<'de> + fmt::Debug>(
        &self,
        url: String,
        body: T,
    ) -> ClientResult<U> {
        let res = self
            .api
            .build_request(&url)
            .bearer_auth(self.token)
            .json(&body)
            .send()
            .await?;

        parse_body(res).await
    }
}
