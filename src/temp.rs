use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::utils::read_strip_newline;

pub struct TemperatureParser {
    path: PathBuf,
}

impl TemperatureParser {
    pub fn new(label: &String) -> Result<Self> {
        let path = discover_temp_path(label)?;
        Ok(Self { path })
    }

    pub fn parse(&self) -> Result<f32> {
        let raw_temp = read_strip_newline(&self.path)?;
        raw_temp
            .parse::<f32>()
            .map(|temp| temp / 1000.0)
            .context("Failed to parse temperature")
    }
}

fn discover_temp_path(label: &String) -> Result<PathBuf> {
    for entry in fs::read_dir("/sys/class/hwmon")? {
        let entry = entry?;
        log::debug!("Scanning {:?}", entry);

        if !entry.file_name().to_str().unwrap().starts_with("hwmon") {
            continue;
        }

        for temp_entry in fs::read_dir(entry.path())? {
            let temp_entry = temp_entry?;
            if !temp_entry.file_name().to_str().unwrap().ends_with("_label") {
                continue;
            }

            let current_label = read_strip_newline(temp_entry.path())?;
            log::debug!("Discovered label {:?}", current_label);
            if &current_label != label {
                continue;
            }

            let temp_file_name = temp_entry
                .file_name()
                .to_str()
                .unwrap()
                .replace("_label", "_input");
            let temp_path = temp_entry.path().with_file_name(&temp_file_name);
            log::debug!("Discovered temp file {:?}", temp_path);
            return Ok(temp_path);
        }
    }

    Err(anyhow!("No hwmon with label {:?}", label))
}
