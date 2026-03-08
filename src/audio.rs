use anyhow::{Context, Result};
use std::fs;

/// Concatenates MP3 byte chunks and writes the combined binary to the specified output file path.
pub fn write_mp3(chunks: Vec<Vec<u8>>, output_path: &str) -> Result<()> {
    let combined: Vec<u8> = chunks.into_iter().flatten().collect();
    fs::write(output_path, combined).context("Failed to write MP3 file")?;
    Ok(())
}
