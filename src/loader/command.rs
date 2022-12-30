use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader};

use subprocess::{Exec, Redirection};
pub fn run_command_and_collect(command: &str) -> Result<String, std::io::Error> {
    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program");

    clearscreen::clear().unwrap();

    let reader = BufReader::new(stream);

    let mut output = String::from("");

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        println!("{}", line);
        let line = strip_color(&line);

        output = output.clone() + &line + "\n";
    });

    Ok(output)
}

fn strip_color(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\\x1B\\[(?:;?[0-9]{1,3})+[mGK]").unwrap();
    }
    RE.replace_all(text, String::from("")).to_string()
}
