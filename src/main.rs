use std::io::{self, Read};

use clap::Parser;
use config::{Args, InputKind, ParserKind};
use loader::command;

/// Contains the analyser for all [`crate::config::ParserKind`]
#[warn(missing_docs)]
pub mod analyser;
/// All cli replated code
#[warn(missing_docs)]
pub mod config;
/// All code related to create .logana-report
#[warn(missing_docs)]
pub mod file;
/// Loads the log for every [`crate::config::InputKind`]
#[warn(missing_docs)]
pub mod loader;
/// The shared type definitions for analyser
#[warn(missing_docs)]
pub mod types;

fn main() {
    let mut args = Args::parse();
    Args::validate(&mut args);
    let mut buffer = String::new();

    match &args.input {
        Some(InputKind::Stdin) => {
            io::stdin().read_to_string(&mut buffer).unwrap_or_default();
            let report = analyse(&args, &buffer);

            file::save_analyse(&report);
        }
        Some(InputKind::Command) => {
            if let Some(command) = &args.command {
                if let Ok(lines) = command::run_command_and_collect(command) {
                    let report = analyse(&args, &lines);
                    file::save_analyse(&report);
                }
            }
        }
        Some(InputKind::Wezterm | InputKind::Tmux) => {
            let content = match &args.input {
                Some(InputKind::Wezterm) => {
                    loader::wezterm::get_wezterm_pane_content(args.target.as_str())
                }
                Some(InputKind::Tmux) => loader::tmux::get_tmux_pane_content(args.target.as_str()),
                _ => None,
            };
            if let Some(content) = content {
                if let Some(report) = loader::split::builds(
                    command::strip_color(content.as_str()).as_str(),
                    &args.splitby,
                )
                .iter()
                .map(|build| analyse(&args, &build))
                .filter(|analyse| !analyse.errors.is_empty())
                .last()
                {
                    file::save_analyse(&report);
                }
            }
        }
        None => {
            println!("There was no --input defined and it could not be guessed");
        }
    };
}

fn analyse(args: &Args, input: &str) -> types::AnalyseReport {
    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            return match args.parser {
                Some(ParserKind::Maven) => analyser::maven::analyse(input, dir),
                Some(ParserKind::Gradle) => analyser::gradle::analyse(input, dir),
                Some(ParserKind::Java) => analyser::java::analyse(input, dir, &args.package),
                Some(ParserKind::KarmaJasmine) => analyser::karma_jasmine::analyse(input, dir),
                Some(ParserKind::Cargo) => analyser::cargo::analyse(input, dir),
                None => {
                    println!("There was no --parser defined and it could not be guessed");

                    types::AnalyseReport::default()
                }
            };
        }
    }
    types::AnalyseReport::default()
}
