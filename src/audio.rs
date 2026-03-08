use anyhow::{Context, Result};
use std::fs;

pub fn write_mp3(chunks: Vec<Vec<u8>>, output_path: &str) -> Result<()> {
    let combined: Vec<u8> = chunks.into_iter().flatten().collect();
    fs::write(output_path, combined).context("Failed to write MP3 file")?;
    Ok(())
}
