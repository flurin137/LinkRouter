use serde::Deserialize;

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
    pub path: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkPattern {
    pub pattern_type: PatternType,
    pub pattern: String,
    pub browser: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PatternType {
    StartsWith,
    Contains
}

pub struct MatchedPattern {
    pub pattern_type: PatternType,
    pub pattern: String,
    pub browser: Option<Browser>,
}
