use axum::{extract::Path, http::{StatusCode, header}, response::Response, body::Body};
use tokio::fs::OpenOptions;
use tokio_util::io::ReaderStream;
use std::path::Path as StdPath;
use crate::utils::response::{ApiResponse, json_response};

pub async fn audio_stream_handler(Path(filename): Path<String>) -> Response {
    let audio_file_path = StdPath::new("audio_files").join(&filename);

    if !audio_file_path.exists() {
        return json_response(StatusCode::NOT_FOUND, ApiResponse::<()>{
            status: "error".into(),
            message: Some("Audio file not found".into()),
            data: None,
        });
    }

    let content_type = match audio_file_path.extension().and_then(|ext| ext.to_str()) {
        Some("mp3") => "audio/mpeg",
        Some("wav") => "audio/wav",
        Some("ogg") => "audio/ogg",
        _ => "application/octet-stream",
    };

    match OpenOptions::new().read(true).open(&audio_file_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(body)
                .unwrap()
        }
        Err(_) => json_response(StatusCode::INTERNAL_SERVER_ERROR, ApiResponse::<()> {
            status: "error".into(),
            message: Some("Failed to open audio file".into()),
            data: None,
        })
    }
}