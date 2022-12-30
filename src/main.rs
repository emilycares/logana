use std::io::{self, Read};

use clap::Parser;
use config::{Args, InputKind, ParserKind};
use loader::command;

pub mod analyser;
pub mod config;
pub mod file;
pub mod loader;
pub mod types;

fn main() {
    let args = Args::parse();
    let mut buffer = String::new();

    match &args.input {
        InputKind::Stdin => io::stdin().read_to_string(&mut buffer).map_or_else(
            |_| {
                let report = analyse(&args, &buffer);

                file::save_analyse(&report);
            },
            |_| println!("Unable to read user input."),
        ),
        InputKind::Command => {
            if let Ok(lines) = command::run_command_and_collect(&args.command) {
                let report = analyse(&args, &lines);
                file::save_analyse(&report);
            }
        }
        InputKind::Tmux => {
            if let Some(content) = loader::fetch::get_tmux_pane_content(args.target.as_str()) {
                if let Some(report) = loader::split::builds(content.as_str(), &args.splitby)
                    .iter()
                    .map(|build| analyse(&args, build))
                    // filter out empty reports
                    .filter(|analyse| !analyse.errors.is_empty())
                    .last()
                {
                    file::save_analyse(&report);
                }
            }
        }
    };
}

fn analyse(args: &Args, input: &str) -> types::AnalyseReport {
    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            return match args.parser {
                ParserKind::Maven => analyser::maven::analyse(input, dir),
                ParserKind::Java => analyser::java::analyse(input, dir, &args.package),
                ParserKind::KarmaJasmine => analyser::karma_jasmine::analyse(input, dir),
                ParserKind::Cargo => analyser::cargo::analyse(input, dir),
                ParserKind::Unknown => {
                    println!("Unknown parser the valid options are \"Cargo\", \"Maven\" and \"KarmaJasmine\"");

                    types::AnalyseReport::default()
                }
            };
        }
    }
    types::AnalyseReport::default()
}
