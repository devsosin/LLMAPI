use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum GeminiModel {
    #[serde(rename(deserialize = "gemini-2.5-flash"))]
    Gemini25Flash,
}

impl From<&str> for GeminiModel {
    fn from(s: &str) -> Self {
        match s {
            "gemini-2.5-flash" => Self::Gemini25Flash,
            _ => Self::Gemini25Flash,
        }
    }
}

impl GeminiModel {
    pub fn as_str(&self) -> &str {
        match self {
            GeminiModel::Gemini25Flash => "gemini-2.5-flash",
        }
    }
}
