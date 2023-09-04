use clap::Parser;
use std::str::FromStr;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The type of log input "Cargo", "Maven" or "KarmaJasmine"
    #[clap(short, long, value_enum, required_if_eq_any([("input", "stdin"), ("input", "tmux"), ("input", "wezterm")]))]
    pub parser: Option<ParserKind>,

    /// The input method that should be used to collect the log.
    #[clap(short, long, value_enum, default_missing_value = "stdin")]
    pub input: Option<InputKind>,

    /// The command to execute
    #[clap(short, long, default_value = "", required_if_eq("input", "command"))]
    pub command: Option<String>,

    /// Additional reference to selected input
    #[clap(short, long, default_value = None, required_if_eq_any([("input", "tmux"), ("input", "wezterm"), ("input", "file")]))]
    pub target: Option<String>,

    /// Your shell PS1 in order to split logs for tmux
    #[clap(short, long, default_value = None, required_if_eq_any([("input", "tmux"), ("input", "wezterm")]))]
    pub splitby: Option<String>,

    /// The java package of your java project
    #[clap(long, default_value = None, required_if_eq("parser", "java"))]
    pub package: Option<String>,

    /// The output method
    #[clap(short, long, default_value = "file", num_args = 0..)]
    pub output: Vec<OutputKind>,

    /// Watch files to rerun
    #[clap(short, long, num_args = 0..,  default_missing_value = "./src")]
    pub watch: Option<String>,

    /// Clear cli bevore ececuting a command
    #[clap(long, default_value = "true")]
    pub clear: bool,

    /// Print the collected log output
    #[clap(long, default_value = "true")]
    pub print_input: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            parser: None,
            input: None,
            command: None,
            target: None,
            splitby: None,
            package: None,
            output: vec![],
            watch: None,
            clear: true,
            print_input: true
        }
    }
}

/// Pecifies witch parser to use
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ParserKind {
    /// The parser for Angular
    Angular,
    /// The parser for cargo
    Cargo,
    /// The parser for gradle
    Gradle,
    /// The parser for Karma with Jasmine
    KarmaJasmine,
    /// The parser for maven
    Maven,
    /// The parser for dune
    Dune,
    /// The parser for eslint
    Eslint,
    /// The parser for go
    Go,
    /// The parser for java
    Java,
    /// The parser for zig
    Zig,
}

impl FromStr for ParserKind {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "maven" => Ok(Self::Maven),
            "java" => Ok(Self::Java),
            "karma-jasmine" => Ok(Self::KarmaJasmine),
            "cargo" | "typos" => Ok(Self::Cargo),
            "gradle" | "./gradlew" => Ok(Self::Gradle),
            "dune" => Ok(Self::Dune),
            "go" => Ok(Self::Go),
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
    /// Take input from a wezterm pane
    Wezterm,
    /// Take input from a command that logana will execute
    Command,
    /// Take input from a specified file
    File,
}

/// Specifies the output type for an analysis
#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq, Eq)]
pub enum OutputKind {
    /// Will writer a .logana-report
    #[default]
    File,
    /// Will write it to stdout
    Stdout,
}

impl Args {
    /// Provides fallbacks for cli arguments
    pub fn validate(args: &mut Self) {
        if args.parser.is_none() {
            let Some(command) = &args.command else {
                return;
            };

            if command.contains(' ') {
                if let Some((first_word, _)) = command.split_once(' ') {
                    args.parser = ParserKind::from_str(first_word).ok();
                }
            } else {
                args.parser = ParserKind::from_str(command).ok();
            }
        }

        if args.input.is_none() && args.command.is_some() {
            args.input = Some(InputKind::Command);
        }
    }
}
