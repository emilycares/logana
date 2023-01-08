use tmux_interface::TmuxCommand;

/// Return the output of a pane
pub fn get_tmux_pane_content(target: &str) -> Option<String> {
    let tmux = TmuxCommand::new();

    match tmux
        .capture_pane()
        .stdout()
        .escape_sequences() // shell colors
        .start_line("-")
        .join()
        .target_pane(target)
        .output()
        .map_or_else(
            |o| Some(o.to_string()),
            |e| {
                Some(e.to_string())
            },
        )
}
