use std::io::{self, Read};

use chrono::Local;
use clap::Parser;
use config::{Args, InputKind, ParserKind};
use loader::command;

pub mod analyser;
pub mod config;
pub mod file;
pub mod loader;
pub mod logvec;
pub mod types;

#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    Args::validate(&mut args);
    let mut buffer = String::new();

    let Some(project_dir) = get_project_dir() else {
        println!("Unable to determine project directory");
        return;
    };

    match &args.input {
        Some(InputKind::Stdin) => {
            if !args.live {
                io::stdin().read_to_string(&mut buffer).map_or_else(
                    |_| {
                        let lines: Vec<&str> = buffer.lines().collect();
                        let lines = lines.as_slice();
                        let report = analyse(&args, &project_dir, lines);

                        file::save_analyse(&report);
                    },
                    |_| println!("Unable to read user input."),
                )
            } else {
                //// https://github.com/Lipen/ctee/blob/master/src/main.rs
                //let mut buf = vec![0];
                //let stdin = io::stdin();
                //let mut stdin = stdin.lock();

                //let mut next_check = Local::now().timestamp_millis();

                //loop {
                //// Read from STDIN
                //let bytes = stdin.read(&mut buf).expect("Unable to read from stdin");

                //// Exit when STDIN is closed
                //if bytes == 0 {
                //break;
                //}

                //// **Very important**, otherwise you can end up with
                //// Heartbleed-esque bugs! I'm chosing to shadow `buf` to
                //// deliberately prevent using it again in this loop.
                //let buf = &buf[..bytes];

                //if let Ok(content) = std::str::from_utf8(buf) {
                //print!("{}", content);
                //}

                //let new_check = Local::now().timestamp_millis();
                //if next_check < new_check {
                //next_check = new_check;

                //if let Some(report) = loader::split::builds(content, &args.splitby)
                //.iter()
                //.map(|build| analyse(&args, project_dir, build))
                //// filter out empty reports
                //.filter(|analyse| !analyse.errors.is_empty())
                //.last()
                //{
                //file::save_analyse(&report);
                //}
                //}
                //}
            }
        }
        Some(InputKind::Command) => {
            if let Some(command) = &args.command {
                if args.live {
                    let (tx, rx) = tokio::sync::mpsc::channel(32);

                    let command = command.clone();
                    tokio::task::spawn(async move {
                        tokio::select! {
                            _ = split_analyse_rx(&args, &project_dir, rx) => {},
                            _ = command::run_command(command, tx) => {}
                        }
                    });
                } else {
                    if let Ok(lines) = command::run_command_and_collect(&command) {
                        let lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
                        let lines = lines.as_slice();
                        let report = analyse(&args, &project_dir, &lines);
                        file::save_analyse(&report);
                    }
                }
            }
        }
        Some(InputKind::Tmux) => {
            if let Some(content) = loader::tmux::get_tmux_pane_content(args.target.as_str()) {
                println!("{}", content);
                tokio::task::spawn(async move {
                    let lines: Vec<&str> = content.lines().collect();
                    let lines = lines.as_slice();
                    split_analyse(&args, &project_dir, lines).await;
                });
            }
        }
    };
}

async fn split_analyse_rx(
    args: &Args,
    project_dir: &str,
    mut rx: tokio::sync::mpsc::Receiver<String>,
) {
    let mut next_check = Local::now().timestamp_millis();

    let mut lines = vec![];
    while let Some(msg) = rx.recv().await {
        lines.push(msg);
        let new_check = Local::now().timestamp_millis() + 5 * 1000;
        if next_check < new_check {
            next_check = new_check;
            let lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
            let lines = lines.as_slice();

            split_analyse(args, project_dir, lines).await;
        }
    } else {
        types::AnalyseReport::default()
    }
}

async fn split_analyse(args: &Args, project_dir: &str, lines: &[&str]) {
    println!("analyse");
    if let Some(report) = loader::split::builds(lines, &args.splitby)
        .iter()
        .map(|build| {
            let lines: Vec<&str> = build.lines().collect();
            let lines = lines.as_slice();

            analyse(&args, project_dir, lines)
        })
        // filter out empty reports
        .filter(|analyse| !analyse.errors.is_empty())
        .last()
    {
        file::save_analyse(&report);
    }
}

fn analyse(args: &Args, project_dir: &str, lines: &[&str]) -> types::AnalyseReport {
    return match args.parser {
        Some(ParserKind::Maven) => analyser::maven::analyse(lines, project_dir),
        Some(ParserKind::Gradle) => analyser::gradle::analyse(lines, project_dir),
        Some(ParserKind::Java) => analyser::java::analyse(lines, project_dir, &args.package),
        Some(ParserKind::KarmaJasmine) => analyser::karma_jasmine::analyse(lines, project_dir),
        Some(ParserKind::Cargo) => analyser::cargo::analyse(lines, project_dir),
        None => {
            println!("There was no --parser defined and it could not be guessed");

            types::AnalyseReport::default()
        }
    };
}

fn get_project_dir<'a>() -> Option<String> {
    if let Ok(dir) = std::env::current_dir() {
        return Some(dir.to_str().unwrap_or(&"".to_string()).to_owned());
    }

    None
}
