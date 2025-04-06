use axum::{Router, routing::post};
use crate::handlers::tts::generate_voice_handler;

pub fn tts_routes() -> Router {
    Router::new()
        .route("/generate-voice", post(generate_voice_handler))  // Handles both text & file upload
}