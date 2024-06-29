use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Alire`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.starts_with("Failed! ") && line.ends_with(" error found at:") {
                errors.extend(parse_fmt_errors(lines, i, &line_len, project_dir));
            }
        }
    }

    errors
}

fn parse_fmt_errors(
    lines: &[&str],
    i: usize,
    line_len: &usize,
    project_dir: &str,
) -> Vec<types::Message> {
    let mut out = vec![];
    for i in i + 1..*line_len {
        if let Some(line) = lines.get(i) {
            if !line.starts_with("- ") {
                break;
            }
            if let Some(error_line) = lines.get(i + 1) {
                if let Some(error_line) = parse_error_line(error_line) {
                    if let Some(error) = parse_fmt_line(line, error_line, project_dir) {
                        out.push(error);
                    }
                }
            }
        }
    }
    out
}

fn parse_error_line(line: &str) -> Option<usize> {
    let mut a = line.splitn(2, " failed on line ");
    a.next()?;
    let rest = a.next()?;
    let mut b = rest.splitn(2, ' ');
    let out: &str = b.next()?;

    let out = out.parse().ok()?;

    Some(out)
}

fn parse_fmt_line(line: &&str, error_line: usize, project_dir: &str) -> Option<types::Message> {
    let line = line.trim_start_matches("- ");
    let mut spl = line.splitn(2, ": ");
    let path = spl.next()?;
    let path = path.replacen("./", &format!("{project_dir}/"), 1);

    Some(types::Message {
        error: spl.next()?.to_string(),
        locations: vec![types::Location {
            path,
            row: error_line,
            col: 0,
        }],
    })
}

#[cfg(test)]
mod tests {
    use crate::{analyser::nix::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn fmt_1() {
        static LOG: &str = include_str!("../../tests/nix_fmt_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "unexpected token at 317..318".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/overlays/default.nix".to_string(),
                    row: 13,
                    col: 0
                }]
            },]
        );
    }
}
