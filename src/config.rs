use clap::Parser;
use std::{str::FromStr};

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The type of log input "Cargo", "Maven" or "KarmaJasmine"
    #[clap(short, long, value_enum)]
    pub parser: Option<ParserKind>,

    /// The input method that should be used to collect the log.
    #[clap(short, long, value_enum, default_missing_value = "stdin")]
    pub input: Option<InputKind>,

    /// The command to execute
    #[clap(short, long, default_value = "", required_if_eq("input", "command"))]
    pub command: Option<String>,

    /// The tmux pane
    #[clap(short, long, default_value = "", required_if_eq("input", "tmux"))]
    pub target: String,

    /// Your shell PS1 in order to split logs for tmux
    #[clap(short, long, default_value = "", required_if_eq("input", "tmux"))]
    pub splitby: String,

    /// The java package of your java project
    #[clap(long, default_value = "", required_if_eq("parser", "java"))]
    pub package: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ParserKind {
    Maven,
    Java,
    KarmaJasmine,
    Cargo,
    Unknown,
}

impl FromStr for ParserKind {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "maven" => Ok(Self::Maven),
            "java" => Ok(Self::Java),
            "karma-jasmine" => Ok(Self::KarmaJasmine),
            "cargo" => Ok(Self::Cargo),
            "typos" => Ok(Self::Cargo),
            &_ => Err(())
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq, Eq)]
pub enum InputKind {
    #[default]
    Stdin,
    Tmux,
    Command,
}

impl Args {
    pub fn validate(args: &mut Args) {
        if args.parser.is_none() {
            let Some(command) = &args.command else {
                return;
            };

            if let Some((first_word, _)) = command.split_once(" ") {
                args.parser = ParserKind::from_str(first_word).ok();
            };
        }

        if args.input.is_none() {
            if args.command.is_some() {
                args.input = Some(InputKind::Command);
            }
        }

        return;
    }
}
