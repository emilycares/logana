use clap::Parser;
use std::{
    io::{self, Read},
    str::FromStr,
    string::ParseError,
};

pub mod analyser;
pub mod types;

/// A build log analysis tool 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The type of log input "Maven" or "KarmaJasmine"
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
        match s.to_ascii_lowercase().as_str() {
            "maven" => Ok(Self::Maven),
            "karmajasmine" => Ok(Self::KarmaJasmine),
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
                    let report = match args.parser {
                        ParserKind::Maven => {
                            analyser::maven::analyse(&buffer, dir)
                        }
                        ParserKind::KarmaJasmine => {
                            analyser::karma_jasmine::analyse(&buffer, dir)
                        }
                        ParserKind::Unknown => {
                            println!("Unknown parser the valid options are \"Maven\" and \"KarmaJasmine\"");

                            types::AnalyseReport {
                                copiler_errors: vec![],
                                test_failures: vec![]
                            }
                        }
                    };

                    println!("{}", report);
                }
            }
        }
        Err(_) => println!("Unable to read user input."),
    }
}
