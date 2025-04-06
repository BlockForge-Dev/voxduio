use std::env;
pub fn get_api_key()-> String{
    env::var("TTS_API_KEY").expect("TTS_API_KEY must be set")
}