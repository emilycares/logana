use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Location {
    pub path: String,
    pub row: usize,
    pub col: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.path, self.row, self.col)
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub error: String,
    pub locations: Vec<Location>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.locations.first() {
            write!(f, "{}|{}", location, self.error)
        } else {
            write!(f, "")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AnalyseReport {
    pub compiler_errors: Vec<Message>,
    pub test_failures: Vec<Message>,
}

impl AnalyseReport {
    pub fn new() -> Self {
        Self {
            compiler_errors: vec![],
            test_failures: vec![],
        }
    }
}

impl Display for AnalyseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let test_failures = self.test_failures.iter();
        self.compiler_errors
            .iter()
            .chain(test_failures)
            .fold(Ok(()), |result, message| {
                result.and_then(|_| writeln!(f, "{}", message))
            })
    }
}

impl Default for AnalyseReport {
    fn default() -> Self {
        Self::new()
    }
}

