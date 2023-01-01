use clap::Parser;
use std::str::FromStr;

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

/// Pecifies witch parser to use 
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ParserKind {
    /// The parser for maven
    Maven,
    /// The parser for java
    Java,
    /// The parser for Karma with Jasmine
    KarmaJasmine,
    /// The parser for cargo
    Cargo,
}

impl FromStr for ParserKind {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "maven" => Ok(Self::Maven),
            "java" => Ok(Self::Java),
            "karma-jasmine" => Ok(Self::KarmaJasmine),
            "cargo" | "typos" => Ok(Self::Cargo),
            &_ => Err(()),
        }
    }
}

/// Specifies the input of the parser
#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq, Eq)]
pub enum InputKind {
    /// Take input from stdin and stderr
    #[default]
    Stdin,
    /// Take input from a tmux pane
    Tmux,
    /// Take input from a command that logana will execute
    Command,
}

impl Args {
    /// Provides fallbacks for cli arguments
    pub fn validate(args: &mut Self) {
        if args.parser.is_none() {
            let Some(command) = &args.command else {
                return;
            };

            if let Some((first_word, _)) = command.split_once(' ') {
                args.parser = ParserKind::from_str(first_word).ok();
            };
        }

        if args.input.is_none() && args.command.is_some() {
            args.input = Some(InputKind::Command);
        }
    }
}
