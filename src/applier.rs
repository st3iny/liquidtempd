use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

use anyhow::{anyhow, Result};
use csscolorparser::Color;

use crate::{Level, MovingAverage, Target};

pub struct LevelApplier {
    interval: Duration,
    levels: Vec<Level>,
    targets: Vec<Target>,
    temps: Arc<Mutex<MovingAverage>>,
}

impl LevelApplier {
    pub fn new(
        interval: Duration,
        levels: Vec<Level>,
        targets: Vec<Target>,
        temps: Arc<Mutex<MovingAverage>>,
    ) -> Self {
        Self {
            interval,
            levels,
            targets,
            temps,
        }
    }

    pub fn run(self) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut last_level: Option<&Level> = None;
            loop {
                sleep(self.interval);

                let avg = self
                    .temps
                    .lock()
                    .expect("Failed to lock moving average mutex")
                    .avg();

                let mut matched_level: Option<&Level> = None;
                for level in &self.levels {
                    if avg >= level.threshold {
                        log::debug!(
                            "Matched level {} to temperature {}",
                            level.color.to_hex_string(),
                            avg
                        );
                        matched_level = Some(level);
                        break;
                    }
                }

                match matched_level {
                    Some(level) => {
                        if last_level != Some(level) {
                            log::info!("Applying color {} to targets", level.color.to_hex_string());
                            self.trigger(&level.color).expect("Failed to run liquidctl");
                            last_level = Some(level);
                        }
                    }
                    None => log::warn!("No level was matched to temperature {}", avg),
                };
            }
        })
    }

    fn trigger(&self, color: &Color) -> Result<()> {
        for target in &self.targets {
            let mut cmd = Command::new("liquidctl");
            cmd.arg(format!("--match={}", target.device));
            cmd.arg("set");
            cmd.arg(&target.channel);
            cmd.arg("color");
            cmd.arg("fixed");
            cmd.arg(&color.to_hex_string()[1..]);
            log::debug!("Spawning {:?}", cmd);
            let status = cmd.status()?;
            if !status.success() {
                return Err(anyhow!("liquidctl reported exit code {:?}", status.code()));
            }
        }

        Ok(())
    }
}
