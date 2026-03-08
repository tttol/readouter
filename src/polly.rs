use anyhow::{Context, Result};
use aws_sdk_polly::{
    types::{Engine, OutputFormat, VoiceId},
    Client,
};

const CHUNK_SIZE_LIMIT: usize = 2800;

/// Splits the given text into chunks that each fit within the Polly SSML size limit (2800 chars).
/// Splits first by double-newline paragraphs, then by sentences if a single paragraph is too large.
pub fn split_text(text: &str) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut current = String::new();
    for paragraph in text.split("\n\n") {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            continue;
        }
        if current.len() + trimmed.len() + 2 > CHUNK_SIZE_LIMIT {
            if !current.is_empty() {
                chunks.push(current.trim().to_string());
                current = String::new();
            }
            // If a single paragraph exceeds the limit, split by sentences
            if trimmed.len() > CHUNK_SIZE_LIMIT {
                for sentence in trimmed.split(". ") {
                    let s = sentence.trim();
                    if s.is_empty() {
                        continue;
                    }
                    if current.len() + s.len() + 2 > CHUNK_SIZE_LIMIT {
                        if !current.is_empty() {
                            chunks.push(current.trim().to_string());
                            current = String::new();
                        }
                        current.push_str(s);
                        current.push_str(". ");
                    } else {
                        current.push_str(s);
                        current.push_str(". ");
                    }
                }
            } else {
                current.push_str(trimmed);
                current.push_str("\n\n");
            }
        } else {
            current.push_str(trimmed);
            current.push_str("\n\n");
        }
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }
    chunks
}

/// Wraps the given text in an SSML `<speak><prosody>` element with the specified speech rate.
/// The speed value (e.g. 0.8) is converted to a percentage string (e.g. "80%").
pub fn to_ssml(text: &str, speed: f64) -> String {
    let rate_percent = (speed * 100.0).round() as u32;
    format!(
        r#"<speak><prosody rate="{}%">{}</prosody></speak>"#,
        rate_percent, text
    )
}

/// Calls Amazon Polly to synthesize the given SSML into MP3 audio bytes using the Neural engine.
pub async fn synthesize(
    client: &Client,
    ssml: &str,
    voice: &str,
) -> Result<Vec<u8>> {
    let voice_id = VoiceId::from(voice);
    let output = client
        .synthesize_speech()
        .engine(Engine::Neural)
        .output_format(OutputFormat::Mp3)
        .voice_id(voice_id)
        .text_type(aws_sdk_polly::types::TextType::Ssml)
        .text(ssml)
        .send()
        .await
        .context("Failed to call Polly synthesize_speech")?;
    let bytes = output
        .audio_stream
        .collect()
        .await
        .context("Failed to read audio stream")?
        .into_bytes()
        .to_vec();
    Ok(bytes)
}
