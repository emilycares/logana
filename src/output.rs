use crate::{
    config::{Args, OutputKind},
    types,
};
use std::{fs::File, io::prelude::*, path::Path};

/// Program output
pub async fn produce(args: &Args, report: &types::AnalyseReport) {
    for kind in &args.output {
        match kind {
            OutputKind::Stdout => println!(
                "{}",
                format!("{report}")
                    .split('\n')
                    .filter(|c| !c.is_empty())
                    .map(|l| format!("logana: {l}"))
                    .fold(String::new(), |a, b| a + &b + "\n")
            ),
            OutputKind::File => file(report),
        }
    }
}

/// Saves the report file
fn file(content: &types::AnalyseReport) {
    let path = Path::new(".logana-report");

    let content = format!("{content}");

    if path.exists() {
        std::fs::remove_file(path).expect("Remove own file");
    }

    let mut file = File::create(path).expect("Create own config file");
    write!(file, "{content}").expect("Write file");
}
