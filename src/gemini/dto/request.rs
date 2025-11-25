use serde::Serialize;

use crate::gemini::types::{Content, Part};

#[derive(Serialize)]
pub struct GeminiRequestBody {
    system_instruction: Content,
    contents: Vec<Content>,
}

impl GeminiRequestBody {
    pub fn new(system_instruction: &str, text: &str) -> Self {
        Self {
            system_instruction: Content::new(vec![Part::new(system_instruction)], None),
            contents: vec![Content::new(vec![Part::new(text)], None)],
        }
    }
}
