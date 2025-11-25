#[cfg(test)]
#[cfg(feature = "gemini")]
mod test {
    use dotenv::dotenv;
    use llm::{GeminiAPI, traits::TextGenerationService};

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv();

        let api = GeminiAPI::from_env();

        let result = api
            .generate_text("gemini-3-pro-preview", "", "hello gemini !")
            .await
            .unwrap();

        println!("{:?}", result);
    }
}
