// handlers/tts.rs

use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::services::tts_engine::generate_tts_audio;


use crate::utils::response::{json_response, ApiResponse};

#[derive(Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice_id: String, // 👈 Accept voice ID from frontend
}

pub async fn generate_voice_handler(Json(payload): Json<TtsRequest>) -> axum::response::Response {
    match generate_tts_audio(&payload.text, &payload.voice_id).await {
        Ok(filename) => json_response(
            StatusCode::OK,
            ApiResponse {
                status: "success".to_string(),
                message: Some("Audio generated successfully".to_string()),
                data: Some(format!("/audio/{}", filename)),
            },
        ),
        Err(e) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiResponse::<()> {
                status: "error".to_string(),
                message: Some(format!("TTS failed: {}", e)),
                data: None,
            },
        ),
    }
}
