use crate::core::types;

const DELIMITERS: [&str; 2] = [": error: ", ": details: "];

/// Contains the analyser code for the [`crate::config::ParserKind::V`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            for delimiter in DELIMITERS {
                if line.contains(delimiter) {
                    if let Some(error) = parse_line(delimiter, line, project_dir) {
                        errors.push(error);
                    }
                }
            }
        }
    }

    errors
}

/// Parse message from line with delimiter
///
/// Gets line input like:
/// "src/main.v:5:1: error: unexpected token `}`, expecting `,`"
///  ------------- -------- ------------------------------------
///  `parse_location`()     |           message
///                       Previous detected delimiter
fn parse_line(delimiter: &str, line: &str, project_dir: &str) -> Option<types::Message> {
    if let Some((path, message)) = line.split_once(delimiter) {
        if let Some(location) = parse_location(path, project_dir) {
            return Some(types::Message {
                error: message.to_string(),
                locations: vec![location],
            });
        }
    }

    None
}

/// Parses location
///
/// Gets location input like:
/// "src/main.v:5:1"
///  ---------- - -
///  path       | |
///             | col
///             row
fn parse_location(location: &str, project_dir: &str) -> Option<types::Location> {
    if let Some((relative_path, line_part)) = location.split_once(':') {
        if let Some((row, col)) = line_part.split_once(':') {
            return Some(types::Location {
                path: format!("{project_dir}/{relative_path}"),
                col: col.parse().unwrap_or_default(),
                row: row.parse().unwrap_or_default(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{analyser::v::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error() {
        static LOG: &str = include_str!("../../tests/v_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "unexpected token `}`, expecting `,`".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/main.v".to_string(),
                    row: 5,
                    col: 1
                }]
            }]
        );
    }

    #[test]
    fn details() {
        static LOG: &str = include_str!("../../tests/v_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "unfinished string literal".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main.v".to_string(),
                        row: 6,
                        col: 1
                    }]
                },
                types::Message {
                    error: "literal started here".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main.v".to_string(),
                        row: 4,
                        col: 10
                    }]
                }
            ]
        );
    }
}
