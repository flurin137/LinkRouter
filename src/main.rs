mod executor;
mod link_sanitizer;
mod models;
mod pattern_matcher;
use anyhow::Result;
use clap::Parser;
use executor::forward_to_browser;
use models::{Configuration, MatchedPattern};
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    link: String,
    configuration: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let link = link_sanitizer::sanitize_link(args.link);

    let configuration = get_configuration(args.configuration)?;
    let patterns = match_patterns(configuration);
    let browser = pattern_matcher::match_pattern(&link, patterns);

    forward_to_browser(&link, browser)
}

fn get_configuration(path: Option<PathBuf>) -> Result<Configuration, anyhow::Error> {
    let config_path = match path {
        Some(path) => path,
        None => {
            let mut config = env::current_exe()?;
            config.pop();
            config.push("Configuration.json");
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
