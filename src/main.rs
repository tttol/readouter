mod audio;
mod cli;
mod extractor;
mod polly;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use cli::Args;
use std::path::PathBuf;

/// Entry point: orchestrates the full pipeline from URL to MP3 output.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Fetching URL: {}", args.url);
    let html = extractor::fetch_html(&args.url).await?;

    let output = args.output.unwrap_or_else(|| {
        let title = extractor::extract_title(&html);
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let filename = format!("output_{}_{}.mp3", title, timestamp);
        let dir = std::env::var("READOUTER_OUTPUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                    .join("readouter")
            });
        std::fs::create_dir_all(&dir).ok();
        dir.join(filename).to_string_lossy().to_string()
    });

    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let bedrock_client = aws_sdk_bedrockruntime::Client::new(&aws_config);

    println!("Extracting article text via Bedrock...");
    let article_text = extractor::extract_article(&bedrock_client, &args.model, &html).await?;

    println!("Article extracted ({} chars). Splitting into chunks...", article_text.len());
    let chunks = polly::split_text(&article_text);
    println!("Split into {} chunk(s).", chunks.len());

    let polly_client = aws_sdk_polly::Client::new(&aws_config);
    let mut audio_chunks: Vec<Vec<u8>> = Vec::new();

    for (i, chunk) in chunks.iter().enumerate() {
        println!("Synthesizing chunk {}/{}...", i + 1, chunks.len());
        let ssml = polly::to_ssml(chunk, args.speed);
        let audio = polly::synthesize(&polly_client, &ssml, &args.voice).await?;
        audio_chunks.push(audio);
    }

    println!("Writing MP3 to {}...", output);
    audio::write_mp3(audio_chunks, &output)?;
    println!("Done! Output: {}", output);

    Ok(())
}
