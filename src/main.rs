mod executor;
mod models;
use anyhow::Result;
use clap::Parser;
use executor::forward_to_browser;
use models::{Browser, Configuration, MatchedPattern};
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

    let patterns = match_patterns(configuration);
    let browser = get_browser_for_link(&args.link, patterns);

    forward_to_browser(&args.link, browser)
}

fn get_browser_for_link(link: &str, patterns: Vec<MatchedPattern>) -> Option<Browser> {
    for pattern in patterns {
        match pattern.pattern_type {
            models::PatternType::StartsWith => {
                if link.starts_with(&pattern.pattern) {
                    return pattern.browser.clone();
                }
            }
            models::PatternType::Contains => {
                if link.contains(&pattern.pattern) {
                    return pattern.browser.clone();
                }
            }
        };
    }

    None
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

            return MatchedPattern {
                browser,
                pattern: d.pattern.clone(),
                pattern_type: d.pattern_type.clone(),
            };
        })
        .collect()
}
