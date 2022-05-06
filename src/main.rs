use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use clap::Parser;
use csscolorparser::Color;
use serde::Deserialize;

use crate::applier::LevelApplier;
use crate::average::MovingAverage;
use crate::checker::CpuTemperatureChecker;
use crate::temp::TemperatureParser;

mod applier;
mod average;
mod checker;
mod temp;
mod utils;

#[derive(Parser)]
#[clap(version)]
pub struct Args {
    /// Path to config file [default: $XDG_CONFIG_HOME/liquidtempd/config.yml]
    #[clap(long)]
    config: Option<String>,

    /// Interval in seconds to apply LED settings
    #[clap(long, default_value_t = 3.0)]
    check_interval: f32,

    /// Interval in seconds to check CPU temperature
    #[clap(long, default_value_t = 15.0)]
    apply_interval: f32,

    /// Window size of CPU temperature moving average in seconds
    #[clap(long, default_value_t = 30.0)]
    window: f32,
}

#[derive(Deserialize)]
pub struct Config {
    pub hwmon_label: String,
    pub levels: Vec<Level>,
    pub targets: Vec<Target>,
}

#[derive(Deserialize)]
pub struct Target {
    pub device: String,
    pub channel: String,
}

#[derive(Deserialize, PartialEq)]
pub struct Level {
    pub color: Color,
    pub threshold: f32,
}

fn main() {
    env_logger::init();

    let mut args = Args::parse();

    assert!(
        args.check_interval > 0.0,
        "Check interval is zero or negative"
    );
    assert!(
        args.apply_interval > 0.0,
        "Apply interval is zero or negative"
    );
    assert!(args.window > 0.0, "Window is zero or negative");

    if args.window < args.check_interval {
        args.window = args.check_interval;
        log::warn!("Setting window equal to check interval of {}s", args.window);
    }

    let config_path = args.config.map(PathBuf::from).unwrap_or_else(|| {
        let config_dirs =
            xdg::BaseDirectories::with_prefix("liquidtempd").expect("Failed to init xdg");
        config_dirs.get_config_file("config.yml")
    });

    let config = fs::read(config_path).expect("Failed to read config");
    let mut config: Config = serde_yaml::from_slice(&config).expect("Failed to parse config");

    config
        .levels
        .sort_by(|a, b| a.threshold.partial_cmp(&b.threshold).unwrap().reverse());

    let window = args.window / args.check_interval;
    let average_temp = Arc::new(Mutex::new(MovingAverage::new(window.ceil() as usize)));

    ctrlc::set_handler(|| {
        exit(1);
    })
        .expect("Failed to start ctrlc handler");

    let checker = CpuTemperatureChecker::new(
        Duration::from_secs_f32(args.check_interval),
        &config.hwmon_label,
        average_temp.clone(),
    )
        .expect("Failed to init checker task");

    let applier = LevelApplier::new(
        Duration::from_secs_f32(args.apply_interval),
        config.levels,
        config.targets,
        average_temp.clone(),
    );

    let tasks = vec![checker.run(), applier.run()];
    for task in tasks {
        task.join().unwrap();
    }
}
