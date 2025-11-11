#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use llm::{LLMAPI, traits::TextGenerationService};

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv();

        let api = LLMAPI::from_env();

        let result = api.generate_text("gpt-5", "hello gpt !").await.unwrap();

        println!("{:?}", result);
    }
}
