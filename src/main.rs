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

    match args.input {
        InputKind::Stdin => match io::stdin().read_to_string(&mut buffer) {
            Ok(_log) => {
                let report = analyse(&args.parser, &buffer);

                file::save_analyse(report);
            }
            Err(_) => println!("Unable to read user input."),
        },
        InputKind::Command => {
            if let Ok(lines) = command::run_command_and_collect(args.command) {
                let report = analyse(&args.parser, &lines);
                file::save_analyse(report);
            }
        }
        InputKind::Tmux => {
            if let Some(content) = loader::fetch::get_tmux_pane_content(args.target.as_str()) {
                let parser = args.parser;
                if let Some(report) = loader::split::split_builds(content.as_str(), &args.splitby)
                    .iter()
                    .map(|build| analyse(&parser, build))
                    // filter out empty reports
                    .filter(|analyse| !analyse.errors.is_empty())
                    .last()
                {
                    file::save_analyse(report);
                }
            }
        }
    };
}

fn analyse(parser: &ParserKind, input: &String) -> types::AnalyseReport {
    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            return match parser {
                ParserKind::Maven => analyser::maven::analyse(input, dir),
                ParserKind::KarmaJasmine => analyser::karma_jasmine::analyse(input, dir),
                ParserKind::Cargo => analyser::cargo::analyse(input, dir),
                ParserKind::Unknown => {
                    println!("Unknown parser the valid options are \"Cargo\", \"Maven\" and \"KarmaJasmine\"");

                    types::AnalyseReport::default()
                }
            };
        } else {
            types::AnalyseReport::default()
        }
    } else {
        types::AnalyseReport::default()
    }
}
