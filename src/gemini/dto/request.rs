use serde::Serialize;

use crate::gemini::types::{Content, Part};

#[derive(Serialize)]
pub struct GeminiRequestBody {
    contents: Vec<Content>,
}

impl GeminiRequestBody {
    pub fn new(text: &str) -> Self {
        Self {
            contents: vec![Content::new(vec![Part::new(text)], None)],
        }
    }
}
