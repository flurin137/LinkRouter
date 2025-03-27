mod models;
use anyhow::Result;
use clap::Parser;
use models::{Browser, Configuration, LinkPattern, MatchedPattern};
use std::{fs, slice::Iter};

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

    println!("Hello {}!", args.link);

    Ok(())
}

pub fn match_patterns(patterns: Vec<LinkPattern>, browsers: Vec<Browser>) -> Vec<MatchedPattern> {
    patterns
        .iter()
        .map(|d| {
            let browser = browsers
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
