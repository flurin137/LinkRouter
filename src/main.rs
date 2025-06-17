mod executor;
mod helpers;
mod link_sanitizer;
mod models;
mod pattern_matcher;

use anyhow::{Result, anyhow};
use clap::Parser;
use executor::forward_to_browser;
use log::info;
use models::{Configuration, MatchedPattern};
use simple_logger::SimpleLogger;
use std::{env, fs, path::PathBuf};

use crate::helpers::shorten;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[arg(short, long)]
    configuration: Option<PathBuf>,
    link: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let level = match args.verbose {
        true => log::LevelFilter::Info,
        false => log::LevelFilter::Warn,
    };
    SimpleLogger::new().with_level(level).init()?;

    let link = link_sanitizer::sanitize_link(args.link);

    let configuration = get_configuration(args.configuration)?;
    let patterns = match_patterns(configuration);
    let browser = pattern_matcher::match_pattern(&link, patterns);

    forward_to_browser(&link, browser)
}

fn get_configuration(path: Option<PathBuf>) -> Result<Configuration, anyhow::Error> {
    let config_path = match path {
        Some(path) => {
            let path_str = path
                .as_os_str()
                .to_str()
                .ok_or_else(|| anyhow!("Unable to get path string"))?;
            info!("Using configuration file at path '{}'", shorten(path_str));
            path
        }
        None => {
            let mut config = env::current_exe()?;
            config.pop();
            config.push("Configuration.json");
            info!("Using default configuration file '{path:?}'");
            config
        }
    };

    let data = fs::read_to_string(config_path)?;
    let configuration: Configuration = serde_json::from_str(&data)?;
    Ok(configuration)
}

pub fn match_patterns(configuration: Configuration) -> Vec<MatchedPattern> {
    configuration
        .link_patterns
        .iter()
        .map(|d| {
            let browser = configuration
                .browsers
                .iter()
                .find(|browser| browser.name == d.browser)
                .map(|d| d.to_owned());

            MatchedPattern {
                browser,
                pattern: d.pattern.clone(),
                pattern_type: d.pattern_type.clone(),
            }
        })
        .collect()
}
