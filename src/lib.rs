use crate::core::config::Args;
use crate::core::types;

/// Contains the analyser for all [`crate::config::ParserKind`]lib
#[warn(missing_docs)]
mod analyser;
/// Loads the log for every [`crate::config::InputKind`]
#[warn(missing_docs)]
mod input;

pub mod core;

/// Runs logana like it is in the cli
pub async fn run(args: core::config::Args, project_dir: &str) {
    let mut args = args;
    core::config::Args::validate(&mut args);
    if args.parser.is_none() {
        println!("There was no -p/--parser defined and it could not be guessed");
    } else {
        input::handle::handle(&args, project_dir).await;
    }
}

/// Evaluate build log and analyse it
/// # Arguments
/// * `project_dir` - A string that is a reference to where this project exists
pub async fn handle_input(args: &Args, project_dir: &str) -> Option<types::AnalyseReport> {
    input::handle::handle_input(args, project_dir).await
}

/// Analyse the input string and returning a AnalyseReport for all parsers
///
/// # Arguments
/// * `source` - A string that provides context. From where the input comes from.
/// * `input` - A string that is a buildlog with no shell escape codes
/// * `project_dir` - A string that is a reference to where this project exists
pub fn analyse(
    args: &Args,
    source: String,
    input: &str,
    project_dir: &str,
) -> types::AnalyseReport {
    input::handle::analyse(args, source, input, project_dir)
}
