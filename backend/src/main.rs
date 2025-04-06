use axum::{Router, routing::{get, post}};
use crate::handlers::{tts::generate_voice_handler, audio::audio_stream_handler};
use hyper::server::conn::AddrStream;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate-voice", post(generate_voice_handler))
        .route("/audio/:filename", get(audio_stream_handler));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
