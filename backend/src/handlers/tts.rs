use axum::{
    extract::{Multipart},
    http::{StatusCode},
    response::{IntoResponse},
};
use crate::{
    services::{tts_engine::generate_voice, file_parser::{InputType, extract_text}},
    utils::response::{json_response, ApiResponse}
};
use uuid::Uuid;

pub async fn generate_voice_handler(multipart: Multipart) -> impl IntoResponse {
    let mut text = String::new();
    let mut voice = None;
    let mut speed = None;

    // Process the multipart form data
    let mut fields = multipart.fields();
    while let Some(field) = fields.next().await {
        match field.name() {
            Some("file") => {
                // Handle file upload
                let file_bytes = field.bytes().await.unwrap();
                match extract_text(InputType::PdfBytes(file_bytes.to_vec())) {
                    Ok(extracted_text) => {
                        text = extracted_text;
                    }
                    Err(_) => {
                        return json_response(StatusCode::BAD_REQUEST, ApiResponse {
                            status: "error".into(),
                            message: Some("Failed to extract text from PDF".into()),
                            data: None,
                        });
                    }
                }
            },
            Some("text") => {
                // Handle raw text input
                let raw_text = field.text().await.unwrap();
                text = raw_text;
            },
            Some("voice") => {
                // Handle voice choice
                voice = Some(field.text().await.unwrap());
            },
            Some("speed") => {
                // Handle speech speed
                match field.text().await.unwrap().parse::<f32>() {
                    Ok(parsed_speed) => speed = Some(parsed_speed),
                    Err(_) => {
                        return json_response(StatusCode::BAD_REQUEST, ApiResponse {
                            status: "error".into(),
                            message: Some("Invalid speed value".into()),
                            data: None,
                        });
                    }
                }
            },
            _ => {}
        }
    }

    // If no text was provided, return error
    if text.is_empty() {
        return json_response(StatusCode::BAD_REQUEST, ApiResponse {
            status: "error".into(),
            message: Some("No valid text or file provided".into()),
            data: None,
        });
    }

    // Call TTS service to generate audio
    match generate_voice(&text, voice, speed).await {
        Ok(audio_url) => {
            json_response(StatusCode::OK, ApiResponse {
                status: "success".into(),
                message: Some("Audio generated successfully".into()),
                data: Some(audio_url),
            })
        }
        Err(err) => {
            // Handle API or system failure
            json_response(StatusCode::INTERNAL_SERVER_ERROR, ApiResponse {
                status: "error".into(),
                message: Some(format!("Failed to generate voice: {}", err)),
                data: None,
            })
        }
    }
}