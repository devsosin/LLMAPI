use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    parts: Vec<Part>,
    role: Option<String>,
}

impl Content {
    pub fn new(parts: Vec<Part>, role: Option<String>) -> Self {
        Self { parts, role }
    }

    pub fn get_text(&self) -> &str {
        &self.parts.last().unwrap().text
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    text: String,
}

impl Part {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}
