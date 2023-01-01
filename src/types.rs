use std::fmt::Display;

/// A file with position
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, PartialEq, Eq)]
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
    /// All errors
    pub errors: Vec<Message>,
}

impl AnalyseReport {
    /// Returns a new `AnalyseReport`
    #[must_use]
    pub const fn new() -> Self {
        Self { errors: vec![] }
    }
}

impl Display for AnalyseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.errors.iter().fold(Ok(()), |result, message| {
            if message.locations.is_empty() {
                result
            } else {
                result.and_then(|_| writeln!(f, "{message}"))
            }
        })
    }
}

impl Default for AnalyseReport {
    fn default() -> Self {
        Self::new()
    }
}
