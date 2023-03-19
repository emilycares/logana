use std::{path::Path, time::Duration};
use tokio::io::AsyncReadExt;

use chrono::Local;
use notify::{PollWatcher, RecursiveMode, Watcher};
use tokio::{fs::read_to_string, io};

use crate::{
    analyser,
    config::{Args, InputKind, ParserKind},
    output,
    types::{self, AnalyseReport},
};

use super::{command, split, tmux, wezterm};

/// Will handle the userinput and call the analyser
pub async fn handle(args: &Args) {
    handle_input(&args).await;

    if args.watch.is_some() {
        handle_watch(&args).await;
    }
}

async fn handle_watch(args: &Args) {
    if let Some(watch) = &args.watch {
        let (tx, rx) = std::sync::mpsc::channel();

        let config = notify::Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_millis(500));

        if let Ok(mut watcher) = PollWatcher::new(tx, config) {
            if watcher
                .watch(Path::new(watch), RecursiveMode::Recursive)
                .is_ok()
            {
                for e in rx.into_iter().flatten() {
                    if let Some(path) = e.paths.first() {
                        if let Ok(meta) = std::fs::metadata(path) {
                            if meta.is_file() {
                                handle_input(&args).await
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_input(args: &Args) {
    let mut buffer = String::new();
    match &args.input {
        Some(InputKind::Stdin) => {
            io::stdin()
                .read_to_string(&mut buffer)
                .await
                .unwrap_or_default();
            let report = analyse(args, "stdin".to_string(), &buffer);

            output::produce(args, &report).await;
        }
        Some(InputKind::Command) => {
            if let Some(command) = &args.command {
                if let Ok(lines) = command::run_command_and_collect(command) {
                    let report = analyse(args, format!("command: {command}"), &lines);
                    output::produce(args, &report).await;
                }
            }
        }
        Some(InputKind::Wezterm | InputKind::Tmux) => {
            let content = match &args.input {
                Some(InputKind::Wezterm) => wezterm::get_wezterm_pane_content(args.target.as_str()),
                Some(InputKind::Tmux) => tmux::get_tmux_pane_content(args.target.as_str()),
                _ => None,
            };
            if let Some(content) = content {
                if let Some(report) = split::builds(
                    command::strip_color(content.as_str()).as_str(),
                    &args.splitby,
                )
                .iter()
                .map(|build| analyse(args, format!("pane: {}", args.target), build))
                .filter(|analyse| !analyse.errors.is_empty())
                .last()
                {
                    output::produce(args, &report).await;
                }
            }
        }
        Some(InputKind::File) => match read_to_string(&args.target).await {
            Ok(content) => {
                let report = analyse(args, format!("file: {}", args.target), &content);
                output::produce(args, &report).await;
            }
            Err(e) => println!("Got the following error wile readindg the target: {e:?}"),
        },
        None => {
            println!("There was no --input defined and it could not be guessed");
        }
    };
}

fn analyse(args: &Args, source: String, input: &str) -> types::AnalyseReport {
    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            let errors = match args.parser {
                Some(ParserKind::Maven) => analyser::maven::analyse(input, dir),
                Some(ParserKind::Gradle) => analyser::gradle::analyse(input, dir),
                Some(ParserKind::Java) => analyser::java::analyse(input, dir, &args.package),
                Some(ParserKind::KarmaJasmine) => analyser::karma_jasmine::analyse(input, dir),
                Some(ParserKind::Cargo) => analyser::cargo::analyse(input, dir),
                None => {
                    println!("There was no --parser defined and it could not be guessed");

                    vec![]
                }
            };

            return types::AnalyseReport {
                project: dir.to_string(),
                date: Local::now(),
                source,
                errors,
            };
        }
    }

    AnalyseReport {
        project: ".".to_string(),
        date: Local::now(),
        source,
        errors: vec![],
    }
}
