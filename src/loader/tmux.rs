use tmux_interface::TmuxCommand;

/// Return the output of a pane
#[must_use]
pub fn get_tmux_pane_content(target: &Option<String>) -> Option<String> {
    let Some(target) = target else {
        return None;
    };

    let tmux = TmuxCommand::new();

    tmux.capture_pane()
        .stdout()
        .escape_sequences() // shell colors
        .start_line("-")
        .join()
        .target_pane(target)
        .output()
        .map_or_else(|o| Some(o.to_string()), |e| Some(e.to_string()))
}
