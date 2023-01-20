use crate::types;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

/// Saves the report file
pub fn save_analyse(content: &types::AnalyseReport) {
    let path = Path::new(".logana-report");

    let content = format!("{content}");

    if path.exists() {
        std::fs::remove_file(path).expect("Remove own file");
    }

    let mut file = File::create(path).expect("Create own config file");
    write!(file, "{content}").expect("Write file");
}

/// Aoppends to the build log
///
/// The pupose of this function is to be able to provide an error location when there is none in
/// the code this is not implemented but somting like this could be used to save the data.
///
/// reutnrs the line number of the first passed line
#[must_use] pub fn append_build_log(lines: &[&str]) -> usize {
    let path = Path::new(".logana-log");

    if !path.exists() {
        let mut file = File::create(path).expect("Create own config file");
        for line in lines {
            write!(file, "{line}").expect("Write file");
        }

        return 0;
    }

    let file = File::open(path).expect("File should be present");
    let buffered = BufReader::new(file);
    let line_count = buffered.lines().count();

    let mut file = File::open(path).expect("File should be present");
    for line in lines {
        write!(file, "{line}").expect("Write file");
    }

    line_count + 1
}
