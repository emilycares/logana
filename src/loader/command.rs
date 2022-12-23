use std::io::{BufRead, BufReader};

use subprocess::{Exec, Redirection};
pub fn run_command_and_collect(command: String) -> Result<String, std::io::Error> {

    let stream = Exec::shell(command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .stream_stdout()
        .expect("To get output from program");

    let reader = BufReader::new(stream);

    let mut output = String::from("");

    reader.lines().for_each(|line| {
        if line.is_ok() {
            let line = line.unwrap();
            println!("{}", line);
            output = output.clone() + &line + "\n";
        }
    });

    Ok(output)
}
