/// Decides with [`crate::config::InputKind`] to choose and executes the analyser
pub mod handle;
/// Loads the log for [`crate::config::InputKind::Command`]
pub mod command;
/// A util function for splitting builds
pub mod split;
/// Loads the log for [`crate::config::InputKind::Tmux`]
pub mod tmux;
/// Loads the log for [`crate::config::InputKind::Wezterm`]
pub mod wezterm;
