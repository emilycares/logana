use subprocess::{Exec, Redirection};

/// Return the output of a pane
#[must_use] pub fn get_wezterm_pane_content(target: &str) -> Option<String> {
    let out = Exec::cmd("wezterm")
        .args(&[
            "cli",
            "get-text",
            "--pane-id",
            target,
            "--start-line",
            "-1000000",
            //"--escapes",
        ])
        .stdout(Redirection::Pipe)
        .capture()
        .expect("Unable to get wezterm output")
        .stdout_str();

    Some(out)
}
