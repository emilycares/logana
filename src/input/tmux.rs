use tmux_interface::{CapturePane, Tmux};

/// Return the output of a pane
#[must_use]
pub fn get_tmux_pane_content(target: &str) -> Option<String> {
    Tmux::with_command(
        CapturePane::new()
            .stdout()
            //.escape_sequences() // shell colors
            .start_line("-")
            .join()
            .target_pane(target),
    )
    .output()
    .map_or_else(
        |o| Some(o.to_string()),
        |_| {
            println!("Unable to read from tmux target: {target}");
            None
        },
    )
}
