use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Dune`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            for err in parse_file_error(i, line, lines, project_dir) {
                errors.push(err);
            }
        }
    }

    errors
}

fn parse_file_error(
    start_index: usize,
    line: &str,
    lines: &[&str],
    project_dir: &str,
) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    if line.starts_with("File ") {
        if let Some(location) = parse_location(line, project_dir) {
            'errors: for y in 1.. {
                let y = start_index + y;
                let Some(line) = lines.get(y) else {
                break 'errors;
            };
                let line = line.trim();

                if line.is_empty() {
                    break 'errors;
                }

                if line.starts_with("Error: ") {
                    errors.push(types::Message {
                        error: line.to_string(),
                        locations: vec![location.clone()],
                    });

                    break 'errors;
                }
            }
        }
    }

    return errors;
}

/// input: File "bin/main.ml", line 21, characters 35-39:
///              -----------        --             --
///              |                  |              |
///              path               row            col
fn parse_location(line: &str, project_dir: &str) -> Option<types::Location> {
    let line = &line[6..line.len() - 1];

    if let Some((path, rest)) = line.split_once("\", line ") {
        if let Some((row, rest)) = rest.split_once(", characters ") {
            if let Some((col, _)) = rest.split_once('-') {
                return Some(types::Location {
                    path: format!("{project_dir}/{path}"),
                    row: row.parse().unwrap_or_default(),
                    col: col.parse::<usize>().unwrap_or_default(),
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::{analyser::dune::analyse, core::types};

    use super::parse_location;

    #[test]
    fn should_find_build_error() {
        static LOG: &str = include_str!("../../tests/dune_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "Error: Unbound record field time".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/bin/main.ml".to_string(),
                    row: 21,
                    col: 35
                }]
            }]
        );
    }

    #[test]
    fn parse_location_test() {
        let result = parse_location(
            "File \"bin/main.ml\", line 21, characters 35-39:",
            "/tmp/project",
        );

        assert_eq!(
            result,
            Option::Some(types::Location {
                path: "/tmp/project/bin/main.ml".to_string(),
                row: 21,
                col: 35
            })
        )
    }
}
