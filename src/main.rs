use std::io::{self, Read};

pub mod analyser;
pub mod types;

fn main() {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_log) => {
            if let Ok(dir) = std::env::current_dir() {
                if let Some(dir) = dir.to_str() {
                    let report = analyser::maven::analyse(&buffer, dir);

                    println!("{}", report);
                }
            }
        }
        Err(_) => todo!(),
    }
}
