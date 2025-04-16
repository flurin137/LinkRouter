mod executor;
mod link_sanitizer;
mod models;
mod pattern_matcher;
use anyhow::Result;
use clap::Parser;
use executor::forward_to_browser;
use models::{Configuration, MatchedPattern};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    link: String,
}

fn main() -> Result<()> {
    let path = "Configuration.json";
    let data = fs::read_to_string(path)?;
    let configuration: Configuration = serde_json::from_str(&data)?;

    let args = Args::parse();

    let link = link_sanitizer::sanitize_link(args.link);

    let patterns = match_patterns(configuration);
    let browser = pattern_matcher::match_pattern(&link, patterns);

    forward_to_browser(&link, browser)
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
