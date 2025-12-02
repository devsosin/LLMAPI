#[cfg(test)]
#[cfg(feature = "gemini")]
mod test {
    use dotenv::dotenv;
    use llm::{
        GeminiAPI, gemini::models::GeminiModel, traits::TextGenerationService,
        types::AgentTextRequest,
    };

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv().ok();

        let api = GeminiAPI::from_env();

        let request = AgentTextRequest::new("You Are a Helpful Bot", "hello gemini !", false);
        let result = api
            .generate_text(GeminiModel::Gemini3ProPreview, request)
            .await
            .unwrap();

        println!("{:?}", result.get_content());
    }
}
