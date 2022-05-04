use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

use anyhow::{Context, Result};

use crate::{MovingAverage, TemperatureParser};

pub struct CpuTemperatureChecker {
    interval: Duration,
    parser: TemperatureParser,
    temps: Arc<Mutex<MovingAverage>>,
}

impl CpuTemperatureChecker {
    pub fn new(
        interval: Duration,
        hwmon_label: &String,
        temps: Arc<Mutex<MovingAverage>>,
    ) -> Result<Self> {
        let parser =
            TemperatureParser::new(hwmon_label).context("Failed to init cpu temperature parser")?;
        Ok(Self {
            interval,
            parser,
            temps,
        })
    }

    pub fn run(self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            self.temps
                .lock()
                .expect("Failed to lock moving average mutex")
                .push(
                    self.parser
                        .parse()
                        .expect("Failed to parse cpu temperature"),
                );
            sleep(self.interval);
        })
    }
}
