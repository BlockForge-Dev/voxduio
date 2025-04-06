use axum::{
    extract::Path,
    response::{IntoResponse, Streaming},
    http::StatusCode,
};
use std::fs::File;
use std::fs;
use std::path::Path as StdPath;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use crate::utils::response::{json_response, ApiResponse};

/// Audio streaming handler
pub async fn audio_stream_handler(Path(filename): Path<String>) -> impl IntoResponse {
    // Path to the audio file
    let audio_file_path = StdPath::new("audio_files").join(filename);

    // Check if the file exists
    if !audio_file_path.exists() {
        return json_response(StatusCode::NOT_FOUND, ApiResponse {
            status: "error".into(),
            message: Some("Audio file not found".into()),
            data: None,
        });
    }

    // Open the audio file asynchronously
    match OpenOptions::new().read(true).open(audio_file_path).await {
        Ok(file) => {
            let stream = tokio::io::ReaderStream::new(file);
            Streaming::new(stream)
        }
        Err(_) => {
            json_response(StatusCode::INTERNAL_SERVER_ERROR, ApiResponse {
                status: "error".into(),
                message: Some("Failed to open audio file".into()),
                data: None,
            })
        }
    }}