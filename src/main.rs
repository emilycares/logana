use clap::Parser;
use config::Args;

/// Contains the analyser for all [`crate::config::ParserKind`]
#[warn(missing_docs)]
pub mod analyser;
/// All cli replated code
#[warn(missing_docs)]
pub mod config;
/// All code related to create .logana-report
#[warn(missing_docs)]
pub mod file;
/// Handles the log input [`crate::config::InputKind`]
#[warn(missing_docs)]
pub mod input;
/// Loads the log for every [`crate::config::InputKind`]
#[warn(missing_docs)]
pub mod loader;
/// Common utlitys for log
#[warn(missing_docs)]
pub mod log;
/// The shared type definitions for analyser
#[warn(missing_docs)]
pub mod types;

#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    Args::validate(&mut args);

    input::handle(args);
}
