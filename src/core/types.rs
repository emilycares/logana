use std::fmt::Display;

use chrono::{DateTime, Local};

/// A file with position
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Location {
    /// File location
    pub path: String,
    /// Row of file
    pub row: usize,
    /// Column of file
    pub col: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.path, self.row, self.col)
    }
}

/// An error message
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Message {
    /// The description of the error
    pub error: String,
    /// All relevant file references of an error
    pub locations: Vec<Location>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.locations.first() {
            write!(f, "{location}|{}", self.error)
        } else {
            write!(f, "")
        }
    }
}

/// A report with all its errors
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyseReport {
    /// Specifies the inputmethod
    pub source: String,
    /// The praject that this analyse belongs to
    pub project: String,
    /// The date that the analyse was done
    pub date: DateTime<Local>,
    /// All errors
    pub errors: Vec<Message>,
}

impl Display for AnalyseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.errors.iter().fold(Ok(()), |result, message| {
            if message.locations.is_empty() {
                result
            } else {
                result.and_then(|()| writeln!(f, "{message}"))
            }
        })
    }
}
