use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Odin`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.starts_with(project_dir) {
                if let Some(error) = parse_line(line) {
                    errors.push(error);
                }
            }
        }
    }

    errors
}

/// Parse message from line with delimiter
///
/// Gets line input like:
/// "/tmp/project/main.odin(7:1) Syntax Error: Expected a statement, got '}'"
///  --------------------------- --------------------------------------------
///  `parse_location`()          message
fn parse_line(line: &str) -> Option<types::Message> {
    let (location, message) = line.split_once(' ')?;
    if let Some(location) = parse_location(location) {
        return Some(types::Message {
            error: message.to_string(),
            locations: vec![location],
        });
    }

    None
}

/// Parses location
///
/// Gets location input like:
/// "/tmp/project/main.odin(7:1)"
///  ---------------------- - -
///  path                   | |
///                         | col
///                         row
fn parse_location(location: &str) -> Option<types::Location> {
    let (path, rest) = location.split_once('(')?;

    let (row, rest) = rest.split_once(':')?;
    let (col, _) = rest.split_once(')')?;

    Some(types::Location {
        path: path.to_string(),
        col: col.parse().unwrap_or_default(),
        row: row.parse().unwrap_or_default(),
    })
}
#[cfg(test)]
mod tests {
    use crate::{analyser::odin::analyse, core::types};
    use pretty_assertions::assert_eq;

    use super::parse_location;

    #[test]
    fn parse_location_basic() {
        let loc = "/tmp/project/main.odin(7:1)";
        let result = parse_location(loc);
        assert_eq!(
            result,
            Some(types::Location {
                path: "/tmp/project/main.odin".to_string(),
                row: 7,
                col: 1
            })
        );
    }

    #[test]
    fn error() {
        static LOG: &str = include_str!("../../tests/odin_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "Syntax Error: Expected a statement, got '}'".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.odin".to_string(),
                        row: 7,
                        col: 1
                    }]
                },
                types::Message {
                    error: "Syntax Error: Only declarations are allowed at file scope, got expression statement".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.odin".to_string(),
                        row: 6,
                        col: 2
                    }]
                },
                types::Message {
                    error: "Error: Cannot convert untyped value '\"Hellope!\"' to 'untyped integer' from 'untyped string'".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.odin".to_string(),
                        row: 6,
                        col: 14
                    }]
                },
                types::Message {
                    error: "Error: 'len' is not supported for 'untyped integer'".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.odin".to_string(),
                        row: 6,
                        col: 14
                    }]
                }
            ]
        );
    }
}
