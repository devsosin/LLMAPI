use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub enum GeminiModel {
    #[serde(rename(deserialize = "gemini-3-pro-preview"))]
    Gemini3ProPreview,
    // $2 / $12 (<200,000 tokens)
    // $4 / $18 (>200,000 tokens)
    #[serde(rename(deserialize = "gemini-3-flash-preview"))]
    Gemini3FlashPreview,
    // $0.50 / $3
}

impl ToString for GeminiModel {
    fn to_string(&self) -> String {
        match self {
            GeminiModel::Gemini3ProPreview => "gemini-3-pro-preview",
            GeminiModel::Gemini3FlashPreview => "gemini-3-flash-preview",
        }
        .into()
    }
}
