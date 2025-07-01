use anyhow::Result;
use regex::Regex;

pub struct SearchEngine;

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine
    }

    pub fn is_regex_valid(pattern: &str) -> bool {
        Regex::new(pattern).is_ok()
    }

    pub fn matches_regex(text: &str, pattern: &str) -> Result<bool> {
        let regex = Regex::new(pattern)?;
        Ok(regex.is_match(text))
    }

    pub fn matches_wildcard(text: &str, pattern: &str) -> bool {
        // 簡単なワイルドカード実装 (*, ?)
        let pattern = pattern
            .replace(".", "\\.")
            .replace("*", ".*")
            .replace("?", ".");
        
        if let Ok(regex) = Regex::new(&format!("^{}$", pattern)) {
            regex.is_match(text)
        } else {
            false
        }
    }
}