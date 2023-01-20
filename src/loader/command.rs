use std::io::{BufRead, BufReader};

use chrono::Local;
use subprocess::{Exec, Redirection};

use crate::{config::Args, file, input, log};

/// Runs the passed command in a shell
pub fn run_command_and_collect(command: &str) -> Result<Vec<String>, std::io::Error> {
    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program");

    clearscreen::clear().expect("The clearscreen lib should be able to clear the screen");

    let reader = BufReader::new(stream);

    let mut lines = vec![];

    reader.lines().for_each(|line| {
        let line = line.expect("Line should be ok");
        println!("{line}");
        let line = log::strip_color(&line);

        lines.push(line + "\n");
    });

    Ok(lines)
}

/// Runs the passed command in a shell
pub fn run_analyse_live(command: String, args: &Args, project_dir: &str) {
    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program");

    clearscreen::clear().expect("The clearscreen lib should be able to clear the screen");

    let reader = BufReader::new(stream);

    let mut next_check = Local::now().timestamp_millis();
    let mut lines = vec![];

    reader.lines().for_each(|line| {
        let line = line.expect("Line should be ok");
        let line = log::strip_color(&line);

        println!("{line}");

        lines.push(line);
        let new_check = next_check + 5000;
        if Local::now().timestamp_millis() > new_check {
            next_check = new_check + 5000;
            let lines: Vec<&str> = lines.iter().map(std::string::String::as_str).collect();
            let lines = lines.as_slice();

            if let Some(report) = input::split_analyse(args, project_dir, lines) {
                file::save_analyse(&report);
            }
        }
    });
}
