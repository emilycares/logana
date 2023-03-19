use clap::Parser;
use config::Args;

/// Contains the analyser for all [`crate::config::ParserKind`]
#[warn(missing_docs)]
pub mod analyser;
/// All cli replated code
#[warn(missing_docs)]
pub mod config;
/// Loads the log for every [`crate::config::InputKind`]
#[warn(missing_docs)]
pub mod input;
/// Output of logana
#[warn(missing_docs)]
pub mod output;
/// The shared type definitions for analyser
#[warn(missing_docs)]
pub mod types;

#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    Args::validate(&mut args);

    if args.parser.is_none() {
        println!("There was no -p/--parser defined and it could not be guessed");
    } else {
        input::handle::handle(&args).await
    }
}
