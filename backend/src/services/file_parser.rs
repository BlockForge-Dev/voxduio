use std::error::Error;
use lopdf::Document;

/// Enum to define the type of input (Raw Text or PDF bytes)
pub enum InputType {
    RawText(String),
    PdfBytes(Vec<u8>),
}

/// Extract text from either raw text or PDF bytes.
pub fn extract_text(input: InputType) -> Result<String, Box<dyn Error>> {
    match input {
        // If raw text is provided, return it directly
        InputType::RawText(text) => Ok(text),

        // If PDF bytes are provided, extract text from the PDF
        InputType::PdfBytes(bytes) => {
            let doc = Document::load_mem(&bytes)?;
            let mut full_text = String::new();

            for (_, page) in doc.get_pages() {
                let content = doc.extract_text(&[page])?;
                full_text.push_str(&content);
            }

            Ok(full_text)
        }
    }
}