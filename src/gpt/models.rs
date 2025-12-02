use serde::Serialize;

#[derive(Serialize)]
pub enum GptModel {
    // Reasoning Models
    #[serde(rename(serialize = "gpt-5.1"))]
    Gpt5_1,

    #[serde(rename(serialize = "gpt-5"))]
    Gpt5,
    #[serde(rename(serialize = "gpt-5 mini"))]
    Gpt5Mini,
    #[serde(rename(serialize = "gpt-5 nano"))]
    Gpt5Nano,

    // Non Reasoning Models
    #[serde(rename(serialize = "gpt-4.1"))]
    Gpt4_1,
    #[serde(rename(serialize = "gpt-4.1-mini"))]
    Gpt4_1Mini,
    #[serde(rename(serialize = "gpt-4.1-nano"))]
    Gpt4_1Nano,
}

impl GptModel {
    pub fn is_reasoning(&self) -> bool {
        match self {
            GptModel::Gpt4_1 => false,
            GptModel::Gpt4_1Mini => false,
            GptModel::Gpt4_1Nano => false,
            _ => true,
        }
    }
}

impl ToString for GptModel {
    fn to_string(&self) -> String {
        match self {
            GptModel::Gpt5_1 => "gpt-5.1",
            GptModel::Gpt5 => "gpt-5",
            GptModel::Gpt5Mini => "gpt-5-mini",
            GptModel::Gpt5Nano => "gpt-5-nano",
            GptModel::Gpt4_1 => "gpt-4.1",
            GptModel::Gpt4_1Mini => "gpt-4.1-mini",
            GptModel::Gpt4_1Nano => "gpt-4.1-nano",
        }
        .into()
    }
}
