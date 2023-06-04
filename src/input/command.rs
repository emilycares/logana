use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader};

use subprocess::{Exec, Redirection};

/// Runs the passed command in a shell
#[must_use]
pub fn run_command_and_collect(command: &str) -> String {
    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program: {command}");

    clearscreen::clear().unwrap_or_default();

    let reader = BufReader::new(stream);

    let mut output = String::new();

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            let line = format!("{line}\n");
            print!("{line}");

            output.push_str(&strip_color(&line));
        } else {
            println!("{line:?}");
        }
    });

    output
}

/// Remove shell colors
#[must_use]
pub fn strip_color(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\\x1B\\[(?:;?[0-9]{1,3})+[mGK]")
            .expect("Unbale to create regex to strip color");
    }
    RE.replace_all(text, String::new()).to_string()
}
