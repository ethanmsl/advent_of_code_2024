//! Error & Result type for {{ project-name | title_case }} of Advent of Code 2024.

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
