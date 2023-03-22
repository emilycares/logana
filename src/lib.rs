/// Contains the analyser for all [`crate::config::ParserKind`]lib
#[warn(missing_docs)]
mod analyser;
/// Loads the log for every [`crate::config::InputKind`]
#[warn(missing_docs)]
mod input;

pub mod core;

pub async fn run(args: core::config::Args) {
    let mut args = args;
    core::config::Args::validate(&mut args);
    if args.parser.is_none() {
        println!("There was no -p/--parser defined and it could not be guessed");
    } else {
        input::handle::handle(&args).await;
    }
}
