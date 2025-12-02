use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub enum GeminiModel {
    #[serde(rename(deserialize = "gemini-2.5-flash"))]
    Gemini25Flash,
    #[serde(rename(deserialize = "gemini-3-pro-preview"))]
    Gemini3ProPreview,
}

impl ToString for GeminiModel {
    fn to_string(&self) -> String {
        match self {
            GeminiModel::Gemini25Flash => "gemini-2.5-flash",
            GeminiModel::Gemini3ProPreview => "gemini-3-pro-preview",
        }
        .into()
    }
}
