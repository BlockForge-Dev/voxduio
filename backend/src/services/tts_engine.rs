use reqwest::Client;
use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};
use uuid::Uuid;

/// Core function: saves audio to a specified path
pub async fn generate_tts_audio_to_path(
    text: &str,
    voice_id: &str,
    output_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")
        .expect("ELEVENLABS_API_KEY must be set in .env");

    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{}/stream",
        voice_id
    );

    let client = Client::new();

    let response = client
        .post(&url)
        .header("xi-api-key", api_key)
        .header("accept", "audio/mpeg")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "text": text,
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.5
            }
        }))
        .send()
        .await?;

    if response.status().is_success() {
        let audio_bytes = response.bytes().await?;

        if let Some(parent) = Path::new(output_path).parent() {
            create_dir_all(parent)?;
        }

        File::create(output_path)?.write_all(&audio_bytes)?;

        Ok(output_path.to_string())
    } else {
        let error_text = response.text().await?;
        Err(format!("TTS failed: {}", error_text).into())
    }
}

/// Wrapper function: saves to a random filename in `audio_files/`
/// Wrapper function: saves to a random filename in `audio_files/`
pub async fn generate_tts_audio(
    text: &str,
    voice_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let filename = format!("{}.mp3", Uuid::new_v4());
    let path = format!("audio_files/{}", filename);

    // ✅ Save the filename to return later
    let audio_filename = filename.clone();

    // Generate the TTS audio and save it to the path
    generate_tts_audio_to_path(text, voice_id, &path).await?;

    // ✅ Return just the filename (not the full path)
    Ok(audio_filename)
}
