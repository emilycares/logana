use clap::Parser;
use std::{
    io::{self, Read},
    str::FromStr,
    string::ParseError,
};

pub mod analyser;
pub mod types;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    parser: ParserKind,
}

#[derive(Parser, Debug)]
enum ParserKind {
    Maven,
    KarmaJasmine,
    Unknown,
}

impl FromStr for ParserKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Maven" => Ok(Self::Maven),
            "KarmaJasmine" => Ok(Self::KarmaJasmine),
            _ => Ok(Self::Unknown),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_log) => {
            if let Ok(dir) = std::env::current_dir() {
                if let Some(dir) = dir.to_str() {
                    match args.parser {
                        ParserKind::Maven => {
                            let report = analyser::maven::analyse(&buffer, dir);

                            println!("{}", report);
                        }
                        ParserKind::KarmaJasmine => {
                            let report = analyser::karma_jasmine::analyse(&buffer, dir);

                            println!("{}", report);
                        }
                        ParserKind::Unknown => {
                            println!("Unknown parser the valid options are \"Maven\" and \"KarmaJasmine\"");
                        }
                    }
                }
            }
        }
        Err(_) => println!("Unable to read user input."),
    }
}
