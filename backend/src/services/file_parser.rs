use std::error::Error;
use lopdf::Document;

/// InputType can be either a path to a PDF file or raw text
pub enum InputType {
    FilePath(String),
    Text(String), //  Make sure this is added
}


/// Extracts text from either a PDF file or returns the raw text input
pub fn extract_text(input: InputType) -> Result<String, Box<dyn Error + Send + Sync>> {
    match input {
        InputType::FilePath(path) => {
            let doc = Document::load(path)?;
            let mut text = String::new();

            for object_id in doc.get_pages().values() {
                if let Ok(content) = doc.extract_text(&[object_id.0]) {
                    text.push_str(&content);
                }
            }

            Ok(text)
        }
        InputType::Text(s) => Ok(s), // Handles plain text input
    }
}
