#[cfg(test)]
#[cfg(feature = "gemini")]
mod test {
    use std::env;

    use dotenv::dotenv;
    use llm::{
        LLMAPI, gemini::models::GeminiModel, traits::TextGenerationService, types::AgentTextRequest,
    };

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv().ok();

        let api = LLMAPI::from_env();
        let api_key =
            env::var("GOOGLE_API_KEY").expect("Failed to load env variable: GOOGLE_API_KEY");
        let api = api.authed_gemini(&api_key);

        let request = AgentTextRequest::new("You Are a Helpful Bot", "hello gemini !", false);
        let result = api
            .generate_text(GeminiModel::Gemini3ProPreview, request)
            .await
            .unwrap();

        println!("{:?}", result.get_content());
    }
}
