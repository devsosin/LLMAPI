#[cfg(test)]
#[cfg(feature = "gpt")]
mod test {
    use std::env;

    use dotenv::dotenv;
    use llm::{
        LLMAPI, gpt::models::GptModel, traits::TextGenerationService, types::AgentTextRequest,
    };

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv().ok();

        let api = LLMAPI::from_env();
        let api_key =
            env::var("OPENAI_API_KEY").expect("Failed to load env variable: OPENAI_API_KEY");
        let api = api.authed_gpt(&api_key);

        let request = AgentTextRequest::new("You Are a Helpful Bot", "hello gpt !", false);
        let result = api
            .generate_text(GptModel::Gpt4_1Nano, request)
            .await
            .unwrap();

        println!("{:?}", result.get_content());
    }
}
