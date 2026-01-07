use serde::Deserialize;

use crate::types::Thinking;
#[derive(Deserialize, Debug)]
pub enum GeminiModel {
    // $2 / $12 (<200,000 tokens)
    // $4 / $18 (>200,000 tokens)
    #[serde(rename(deserialize = "gemini-3-pro-preview"))]
    Gemini3ProPreview,

    // $0.50 / $3
    #[serde(rename(deserialize = "gemini-3-flash-preview"))]
    Gemini3FlashPreview,

    #[serde(rename(deserialize = "gemini-2.5-flash-image"))]
    Gemini2_5FlashImage,
}

impl ToString for GeminiModel {
    fn to_string(&self) -> String {
        match self {
            GeminiModel::Gemini3ProPreview => "gemini-3-pro-preview",
            GeminiModel::Gemini3FlashPreview => "gemini-3-flash-preview",
            GeminiModel::Gemini2_5FlashImage => "gemini-2.5-flash-image",
        }
        .into()
    }
}

impl GeminiModel {
    pub fn check_thinking(&self, thinking: &Thinking) -> bool {
        match self {
            GeminiModel::Gemini3ProPreview => {
                thinking.eq(&Thinking::High) || thinking.eq(&Thinking::Low)
            }
            GeminiModel::Gemini2_5FlashImage => todo!(),
            _ => true,
        }
    }
}
