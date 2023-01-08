use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader};

use subprocess::{Exec, Redirection};

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
        let line = strip_color(&line);

        lines.push(line.to_string() + "\n");
    });

    Ok(lines)
}

/// Runs the passed command in a shell
pub async fn run_command(command: String, tx: tokio::sync::mpsc::Sender<String>) {
    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program");

    clearscreen::clear().unwrap();

    let reader = BufReader::new(stream);

    let mut output = String::from("");

    for line in reader.lines() {
        let line = line.expect("Line should be ok");
        println!("{line}");
        let line = strip_color(&line);

        output = output.clone() + &line + "\n";

        match tx.send(output.clone()).await {
            Ok(_) => {}
            Err(e) => {
                println!("Falied to send message: {}", e)
            }
        }
    }
}

fn strip_color(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\\x1B\\[(?:;?[0-9]{1,3})+[mGK]").unwrap();
    }
    RE.replace_all(text, String::from("")).to_string()
}
