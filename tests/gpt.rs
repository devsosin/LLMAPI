#[cfg(test)]
#[cfg(feature = "gpt")]
mod test {
    use dotenv::dotenv;
    use llm::{
        GptAPI, gpt::models::GptModel, traits::TextGenerationService, types::AgentTextRequest,
    };

    #[tokio::test]
    async fn test_gpt_text_generation() {
        dotenv().ok();

        let api = GptAPI::from_env();

        let request = AgentTextRequest::new("You Are a Helpful Bot", "hello gpt !", false);
        let result = api
            .generate_text(GptModel::Gpt4_1Nano, request)
            .await
            .unwrap();

        println!("{:?}", result.get_content());
    }
}
