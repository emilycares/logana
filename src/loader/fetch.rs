use tmux_interface::TmuxCommand;

/// Return the output of a pane
pub fn get_tmux_pane_content(target: &str) -> Option<String> {
    let tmux = TmuxCommand::new();

    match tmux
        .capture_pane()
        .stdout()
        //.escape_sequences()
        .start_line("-")
        .join()
        .target_pane(target)
        .output()
    {
        Ok(o) => Some(o.to_string()),
        Err(_) => {
            println!("Unable to read from tmux target: {}", target);
            None
        }
    }
}
