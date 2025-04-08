use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    http::StatusCode,
    extract::Multipart,
};
use tokio::net::TcpListener;
use dotenv::dotenv;
use std::env;
use tower_http::services::fs::ServeDir;
 // ✅ Add this

mod handlers;
mod services;
mod utils;

use handlers::{
    tts::generate_voice_handler,
    audio::audio_stream_handler,
    pdf::upload_pdf_handler,
};

/// Health check route
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let elevenlabs_api_key = env::var("ELEVENLABS_API_KEY")
        .expect("ELEVENLABS_API_KEY not set in .env");

    println!("API key loaded (starts with): {}", &elevenlabs_api_key[0..5]);

    let app = Router::new()
        .route("/generate-voice", post(generate_voice_handler))
        .route("/upload-pdf", post(upload_pdf_handler))
        .route("/health", get(health_check))
        // ✅ Serve static audio files from /audio/
        .nest_service("/audio", ServeDir::new("audio_files"));
    

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
