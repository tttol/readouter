mod audio;
mod cli;
mod extractor;
mod polly;

use anyhow::Result;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Fetching URL: {}", args.url);
    let html = extractor::fetch_html(&args.url)?;

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

    println!("Writing MP3 to {}...", args.output);
    audio::write_mp3(audio_chunks, &args.output)?;
    println!("Done! Output: {}", args.output);

    Ok(())
}
