use std::{path::Path, time::Duration};
use tokio::io::AsyncReadExt;

use chrono::Local;
use notify::{PollWatcher, RecursiveMode, Watcher};
use tokio::{fs::read_to_string, io};

use crate::{
    analyser,
    core::{
        config::{Args, InputKind, ParserKind},
        output, types,
    },
};

use super::{command, split, tmux, wezterm};

/// Will handle the userinput and call the analyser
/// This will also handle the wach flag
pub async fn handle(args: &Args, project_dir: &str) {
    if let Some(report) = handle_input(args, project_dir).await {
        output::produce(args, &report);
    }

    if args.watch.is_some() {
        handle_watch(args, project_dir).await;
    }
}

async fn handle_watch(args: &Args, project_dir: &str) {
    if let Some(watch) = args.watch.clone() {
        let (otx, orx) = tokio::sync::watch::channel("watch");

        tokio::spawn(async move {
            listen_fs(&watch, otx);
        });

        let args = args.clone();
        let project_dir = project_dir.to_string();
        tokio::spawn(async move {
            act_fs(args, project_dir, orx).await;
        });

        // Prevent the main task from exiting prematurely
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

async fn act_fs(args: Args, project_dir: String, mut orx: tokio::sync::watch::Receiver<&str>) {
    loop {
        let Ok(has_changed) = orx.has_changed() else {
            break;
        };
        if !has_changed {
            continue;
        }
        orx.borrow_and_update();
        if let Some(report) = handle_input(&args, &project_dir).await {
            output::produce(&args, &report);
        }
    }
}

fn listen_fs(watch: &String, otx: tokio::sync::watch::Sender<&str>) {
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
                            let _ = otx.send("");
                        }
                    }
                }
            }
        }
    }
}

/// Evaluate build log and analyse it
/// # Arguments
/// * `project_dir` - A string that is a reference to where this project exists
pub async fn handle_input(args: &Args, project_dir: &str) -> Option<types::AnalyseReport> {
    let mut buffer = String::new();
    match &args.input {
        Some(InputKind::Stdin) => {
            io::stdin()
                .read_to_string(&mut buffer)
                .await
                .unwrap_or_default();
            let report = analyse(args, "stdin".to_string(), &buffer, project_dir);

            return Some(report);
        }
        Some(InputKind::Command) => {
            if let Some(command) = &args.command {
                let lines = command::run_command_and_collect(command);
                let report = analyse(args, format!("command: {command}"), &lines, project_dir);
                return Some(report);
            }
        }
        Some(InputKind::Wezterm | InputKind::Tmux) => {
            let Some(target) = &args.target else {
                println!("The required argument target is missing");
                return None;
            };
            let target = target.as_str();
            let content = match &args.input {
                Some(InputKind::Wezterm) => wezterm::get_wezterm_pane_content(target),
                Some(InputKind::Tmux) => tmux::get_tmux_pane_content(target),
                _ => None,
            };
            if let Some(content) = content {
                let Some(splitby) = &args.splitby else {
                    println!("The required argument splitby s missing");
                    return None;
                };
                if let Some(report) =
                    split::builds(command::strip_color(content.as_str()).as_str(), splitby)
                        .iter()
                        .map(|build| analyse(args, format!("pane: {target}"), build, project_dir))
                        .filter(|analyse| !analyse.errors.is_empty())
                        .last()
                {
                    return Some(report);
                }
            }
        }
        Some(InputKind::File) => {
            let Some(target) = &args.target else {
                println!("The required argument target is missing");
                return None;
            };
            let target = target.as_str();
            match read_to_string(target).await {
                Ok(content) => {
                    let report = analyse(args, format!("file: {target}"), &content, project_dir);
                    return Some(report);
                }
                Err(e) => {
                    println!("Got the following error wile readindg the target: {e:?}");
                }
            }
        }
        None => {
            println!("There was no --input defined and it could not be guessed");
        }
    };

    None
}

/// Analyse the input string and returning a `AnalyseReport` for all parsers
///
/// # Arguments
/// * `source` - A string that provides context. From where the input comes from.
/// * `input` - A string that is a buildlog with no shell escape codes
/// * `project_dir` - A string that is a reference to where this project exists
pub fn analyse(
    args: &Args,
    source: String,
    input: &str,
    project_dir: &str,
) -> types::AnalyseReport {
    let errors = match args.parser {
        Some(ParserKind::Alire) => analyser::alire::analyse(input, project_dir),
        Some(ParserKind::Angular) => analyser::angular::analyse(input, project_dir),
        Some(ParserKind::Biome) => analyser::biome::analyse(input, project_dir),
        Some(ParserKind::Cargo) => analyser::cargo::analyse(input, project_dir),
        Some(ParserKind::Clang) => analyser::clang::analyse(input, project_dir),
        Some(ParserKind::Dune) => analyser::dune::analyse(input, project_dir),
        Some(ParserKind::Eslint) => analyser::eslint::analyse(input, project_dir),
        Some(ParserKind::Go) => analyser::go::analyse(input, project_dir),
        Some(ParserKind::Gradle) => analyser::gradle::analyse(input, project_dir),
        Some(ParserKind::Java) => {
            if let Some(package) = &args.package {
                let package = package.as_str();
                analyser::java::analyse(input, project_dir, package)
            } else {
                println!("The argument package is required for java");
                vec![]
            }
        }
        Some(ParserKind::KarmaJasmine) => analyser::karma_jasmine::analyse(input, project_dir),
        Some(ParserKind::Maven) => analyser::maven::analyse(input, project_dir),
        Some(ParserKind::Nix) => analyser::nix::analyse(input, project_dir),
        Some(ParserKind::Odin) => analyser::odin::analyse(input, project_dir),
        Some(ParserKind::V) => analyser::v::analyse(input, project_dir),
        Some(ParserKind::Zig) => analyser::zig::analyse(input, project_dir),
        None => {
            println!("There was no --parser defined and it could not be guessed");

            vec![]
        }
    };

    types::AnalyseReport {
        project: project_dir.to_string(),
        date: Local::now(),
        source,
        errors,
    }
}
