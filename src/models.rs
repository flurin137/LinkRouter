use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub browsers: Vec<Browser>,
    pub link_patterns: Vec<LinkPattern>,
}

#[derive(Deserialize, Debug)]
pub struct Browser {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct LinkPattern {
    pub pattern_type: String,
    pub pattern: String,
    pub browser: String,
}

#[derive(Deserialize, Debug)]
pub enum PatternType {
    StartsWith,
}
