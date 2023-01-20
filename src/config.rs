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
    pub target: Option<String>,

    /// Incremental analyse
    #[clap(short, long, default_value = "false")]
    pub live: bool,

    /// Your shell PS1 in order to split logs for tmux
    #[clap(short, long, default_value = "", required_if_eq("input", "tmux"))]
    pub splitby: Option<String>,

    /// The java package of your java project
    #[clap(long, default_value = "", required_if_eq("parser", "java"))]
    pub package: Option<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            parser: Some(ParserKind::Cargo),
            input: None,
            command: None,
            target: None,
            live: false,
            splitby: None,
            package: None,
        }
    }
}

/// Pecifies witch parser to use
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ParserKind {
    /// The parser for maven
    Maven,
    /// The parser for gradle
    Gradle,
    /// The parser for java exepcions
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
            "cargo" | "typos" | "typos.exe" => Ok(Self::Cargo),
            "gradle" | "./gradlew" => Ok(Self::Gradle),
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

fn get_splitby_from_parserkind(parser: Option<ParserKind>) -> Option<String> {
    match parser {
        Some(ParserKind::KarmaJasmine) => {
            Some("Browser application bundle generation complete".to_string())
        }
        _ => None,
    }
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

        if let Some(splitby) = &args.splitby {
            if splitby.is_empty() {
                args.splitby = None;
            }
        }

        if args.splitby.is_none() && args.parser.is_some() {
            args.splitby = get_splitby_from_parserkind(args.parser.as_ref().cloned());
        }

        if args.input.is_none() && args.command.is_some() {
            args.input = Some(InputKind::Command);
        }
    }
}
