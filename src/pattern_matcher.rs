use regex::Regex;

use crate::models::{Browser, MatchedPattern, PatternType};

pub fn match_pattern(link: &str, patterns: Vec<MatchedPattern>) -> Option<Browser> {
    for pattern in patterns {
        match pattern.pattern_type {
            PatternType::StartsWith => {
                if link.starts_with(&pattern.pattern) {
                    return pattern.browser.clone();
                }
            }
            PatternType::Contains => {
                if link.contains(&pattern.pattern) {
                    return pattern.browser.clone();
                }
            }
            PatternType::Regex => {
                if let Ok(regex) = Regex::new(&pattern.pattern) {
                    if regex.is_match(link) {
                        return pattern.browser.clone();
                    }
                };
            }
        };
    }
    None
}
