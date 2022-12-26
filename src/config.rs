use clap::Parser;

/// A build log analysis tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The type of log input "Cargo", "Maven" or "KarmaJasmine"
    #[clap(short, long, value_enum)]
    pub parser: ParserKind,

    /// The input method that should be used to collect the log.
    #[clap(short, long, value_enum, default_missing_value = "stdin")]
    pub input: InputKind,

    /// The command to execute
    #[clap(short, long, default_value = "", required_if_eq("input", "command"))]
    pub command: String,

    /// The tmux pane
    #[clap(short, long, default_value = "", required_if_eq("input", "tmux"))]
    pub target: String,

    /// Your shell PS1 in order to split logs for tmux
    #[clap(short, long, default_value = "", required_if_eq("input", "tmux"))]
    pub splitby: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ParserKind {
    Maven,
    KarmaJasmine,
    Cargo,
    Java,
    Unknown,
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum InputKind {
    #[default]
    Stdin,
    Tmux,
    Command,
}
