use serde::Serialize;

#[derive(Serialize)]
pub enum GptModel {
    #[serde(rename(serialize = "gpt-5"))]
    Gpt5,
}

impl From<&str> for GptModel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gpt-5" => Self::Gpt5,
            _ => Self::Gpt5,
        }
    }
}
