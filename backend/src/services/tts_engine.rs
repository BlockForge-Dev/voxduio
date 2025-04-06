use reqwest::{Client, Error};
use std::fs::{File, create_dir_all};
use std::path::Path;
use uuid::Uuid;
use std::io::Write;

/// Coqui API URL
const COQUI_API_URL: &str = "https://api.coqui.ai/tts";  // Replace with the actual Coqui API endpoint

/// Generate voice from the provided text using Coqui TTS API
pub async fn generate_voice(text: &str, voice: Option<String>, speed: Option<f32>) -> Result<String, Error> {
    let client = Client::new();
    
    // Build the request body
    let mut params = std::collections::HashMap::new();
    params.insert("text", text.to_string());
    if let Some(v) = voice {
        params.insert("voice", v);
    }
    if let Some(s) = speed {
        params.insert("speed", s.to_string());
    }

    // Send POST request to Coqui TTS API
    let response = client
        .post(COQUI_API_URL)
        .json(&params)
        .send()
        .await?;

    if response.status().is_success() {
        // Get the audio file (this assumes that the API returns the audio as raw bytes)
        let audio_bytes = response.bytes().await?;

        // Save audio to file system
        let audio_filename = format!("{}.mp3", Uuid::new_v4());
        let audio_path = Path::new("audio_files").join(&audio_filename);

        // Create audio_files directory if it doesn't exist
        create_dir_all("audio_files").unwrap();

        let mut file = File::create(audio_path.clone())?;
        file.write_all(&audio_bytes)?;

        // Return the path or URL of the saved file
        Ok(format!("/audio/{}", audio_filename))
    } else {
        Err(reqwest::Error::new(response.status(), "Failed to generate voice"))
    }
}