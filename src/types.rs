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
