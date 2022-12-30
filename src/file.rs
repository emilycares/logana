use crate::types;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn save_analyse(content: &types::AnalyseReport) {
    let path = Path::new(".logana-report");

    let content = format!("{content}");

    if path.exists() {
        std::fs::remove_file(path).expect("Remove own file");
    }

    let mut file = File::create(path).expect("Create own config file");
    write!(file, "{content}").expect("Write file");
}
