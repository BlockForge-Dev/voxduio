use axum::{
    extract::Multipart,
    response::IntoResponse,
    http::StatusCode,
};
use crate::services::file_parser::{extract_text, InputType};
use crate::services::tts_engine::generate_tts_audio;
use crate::utils::response::{json_response, ApiResponse};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn upload_pdf_handler(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or_default().to_string();

        if name == "file" {
            // Ensure upload folder exists
            std::fs::create_dir_all("uploads").unwrap();

            // Save file locally
            let data = field.bytes().await.unwrap();
            let filename = format!("{}.pdf", Uuid::new_v4());
            let filepath = format!("./uploads/{}", filename);
            let mut file = File::create(&filepath).unwrap();
            file.write_all(&data).unwrap();

            // Extract text from PDF
            match extract_text(InputType::FilePath(filepath.clone())) {
                Ok(text) => {
                    let  voice_id = "21m00Tcm4TlvDq8ikWAM".to_string(); // Replace or pass dynamically if needed

                    // Truncate based on current quota (e.g., 9763 chars)
                    let max_chars = 500;
                    let truncated_text = &text[..text.len().min(max_chars)];

                    // Generate TTS audio
                    match generate_tts_audio(truncated_text, &voice_id).await {
                        Ok(audio_filename) => {
                            let audio_url = format!("/audio/{}", audio_filename);
                            return json_response(
                                StatusCode::OK,
                                ApiResponse {
                                    status: "success".to_string(),
                                    message: Some("PDF processed and audio generated".to_string()),
                                    data: Some(audio_url),
                                },
                            );
                        }
                        Err(e) => {
                            return json_response(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                ApiResponse::<()> {
                                    status: "error".to_string(),
                                    message: Some(format!("TTS generation failed: {}", e)),
                                    data: None,
                                },
                            );
                        }
                    }
                }
                Err(e) => {
                    return json_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiResponse::<()> {
                            status: "error".to_string(),
                            message: Some(format!("Text extraction failed: {}", e)),
                            data: None,
                        },
                    );
                }
            }
        }
    }

    json_response(
        StatusCode::BAD_REQUEST,
        ApiResponse::<()> {
            status: "error".to_string(),
            message: Some("No PDF file provided".to_string()),
            data: None,
        },
    )
}
