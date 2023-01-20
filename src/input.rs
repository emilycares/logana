use std::io::{self, Read};

use crate::config::{Args, InputKind, ParserKind};
use crate::loader::command;
use crate::{analyser, file, loader, types};

/// Handels log input
pub fn handle(args: Args) {
    let Some(project_dir) = get_project_dir() else {
        println!("Unable to determine project directory");
        return;
    };
    match &args.input {
        None => {
            println!("No input type was provided");
        }
        Some(InputKind::Stdin) => {
            if args.live {
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
            } else {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer).map_or_else(
                    |_| {
                        let lines: Vec<&str> = buffer.lines().collect();
                        let lines = lines.as_slice();
                        let report = analyse(&args, &project_dir, lines);

                        file::save_analyse(&report);
                    },
                    |_| println!("Unable to read user input."),
                );
            }
        }
        Some(InputKind::Command) => {
            if let Some(command) = &args.command {
                if args.live {
                    let command = command.clone();
                    command::run_analyse_live(command, &args, &project_dir);
                } else if let Ok(lines) = command::run_command_and_collect(command) {
                    let lines: Vec<&str> = lines.iter().map(std::string::String::as_str).collect();
                    let lines = lines.as_slice();
                    let report = analyse(&args, &project_dir, lines);
                    file::save_analyse(&report);
                }
            }
        }
        Some(InputKind::Tmux) => {
            if let Some(content) = loader::tmux::get_tmux_pane_content(&args.target) {
                println!("{content}");
                tokio::task::spawn(async move {
                    let lines: Vec<&str> = content.lines().collect();
                    let lines = lines.as_slice();
                    if let Some(report) = split_analyse(&args, &project_dir, lines) {
                        file::save_analyse(&report);
                    }
                });
            }
        }
    };
}

/// Splits lines into builds and analyses the lates build
#[must_use] pub fn split_analyse(
    args: &Args,
    project_dir: &str,
    lines: &[&str],
) -> Option<types::AnalyseReport> {
    let Some(splitby) = &args.splitby else {
        println!("Unable to split builds on splitby was passed and it could not be guessed succesfuly");
        return None;
    };

    return loader::split::builds(lines, splitby)
        .iter()
        .map(|build| analyse(args, project_dir, build))
        //filter out empty reports
        .filter(|analyse| !analyse.errors.is_empty())
        .last();
}

fn analyse(args: &Args, project_dir: &str, lines: &[&str]) -> types::AnalyseReport {
    return match args.parser {
        Some(ParserKind::Maven) => analyser::maven::analyse(lines, project_dir),
        Some(ParserKind::Gradle) => analyser::gradle::analyse(lines, project_dir),
        Some(ParserKind::Java) => analyser::java::analyse(
            lines,
            project_dir,
            args
                .package
                .as_ref()
                .expect("The analyser java neads a package"),
        ),
        Some(ParserKind::KarmaJasmine) => analyser::karma_jasmine::analyse(lines, project_dir),
        Some(ParserKind::Cargo) => analyser::cargo::analyse(lines, project_dir),
        None => {
            println!("There was no --parser defined and it could not be guessed");

            types::AnalyseReport::default()
        }
    };
}

fn get_project_dir() -> Option<String> {
    if let Ok(dir) = std::env::current_dir() {
        return Some(dir.to_str().unwrap_or("").to_owned());
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{Args, ParserKind},
        input::split_analyse,
        types,
    };

    #[test]
    fn split_analyse_should_work_as_intended() {
        static LOG: &str = include_str!("../tests/split_analyse_1.log");
        let lines: Vec<&str> = LOG.lines().collect();
        let lines = lines.as_slice();

        let result = split_analyse(
            &Args {
                parser: Some(ParserKind::KarmaJasmine),
                splitby: Some("Browser application bundle generation complete".to_string()),
                ..Default::default()
            },
            "/tmp/project",
            lines,
        );

        assert_eq!(result, Some(types::AnalyseReport { errors: vec![
            types::Message {
                error: "2 Cannot read property 'component' of undefined".to_string(),
                locations: vec![
                    types::Location { 
                        path: "/tmp/project/src/app/components/layout/main/command-info-dialog-modal/command-info-dialog-modal.component.ts".to_string(),
                        row: 83,
                        col: 1 
                    }
                ]
            }]
        }));
    }

    #[test]
    fn split_analyse_should_take() {
        static LOG: &str = include_str!("../tests/split_analyse_2.log");
        let lines: Vec<&str> = LOG.lines().collect();
        let lines = lines.as_slice();

        let result = split_analyse(
            &Args {
                parser: Some(ParserKind::KarmaJasmine),
                splitby: Some("Browser application bundle generation complete".to_string()),
                ..Default::default()
            },
            "/tmp/project",
            lines,
        );

        assert_eq!(
            result,
            Some(types::AnalyseReport {
                errors: vec![types::Message {
                    error: "2 Cannot read property 'component' of undefined".to_string(),
                    locations: vec![]
                }]
            })
        );
    }
}
