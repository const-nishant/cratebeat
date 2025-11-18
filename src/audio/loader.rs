// Load WAV samples
use anyhow::{Context, Result};
use std::fs;

pub fn load_file_bytes(path: &str) -> Result<Vec<u8>> {
    let b = fs::read(path).with_context(|| format!("Failed to read sound file: {}", path))?;
    Ok(b)
}
