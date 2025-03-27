use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub browsers: Vec<Browser>,
    pub link_patterns: Vec<LinkPattern>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkPattern {
    pub pattern_type: String,
    pub pattern: String,
    pub browser: String,
}

#[derive(Deserialize, Debug)]
pub enum PatternType {
    StartsWith,
}

pub struct MatchedPattern {
    pub pattern_type: String,
    pub pattern: String,
    pub browser: Option<Browser>,
}
