use std::fs;
use std::path::Path;

pub fn read_strip_newline(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let raw = fs::read_to_string(path)?;
    Ok(raw.split_ascii_whitespace().next().unwrap().to_owned())
}
