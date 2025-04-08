use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const ELEVENLABS_API_KEY: &str = "your-api-key-here";
const VOICE_ID: &str = "EXAVITQu4vr4xnSDxMaL"; // default voice, you can change this

pub async fn generate_tts_audio(text: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{VOICE_ID}"
    );

    let payload = serde_json::json!({
        "text": text,
        "model_id": "eleven_monolingual_v1",
        "voice_settings": {
            "stability": 0.5,
            "similarity_boost": 0.75
        }
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("xi-api-key", ELEVENLABS_API_KEY)
        .header("Content-Type", "application/json")
        .header("Accept", "audio/mpeg")
        .json(&payload)
        .send()
        .await?;

    let bytes = response.bytes().await?;

    let output_path = Path::new("audio_files").join(filename);
    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;

    Ok(())
}
  