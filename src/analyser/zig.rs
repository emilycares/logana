use crate::core::types;

const DELIMITERS: [&str; 2] = [": error: ", ": note: "];

/// Contains the analyser code for the [`crate::config::ParserKind::Maven`]
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

fn parse_location(location: &str, project_dir: &str) -> Option<types::Location> {
    if let Some((relative_path, line_part)) = location.split_once(':') {
        if let Some((row, col)) = line_part.split_once(':') {
            return Some(types::Location {
                path: project_dir.to_string() + "/" + relative_path,
                col: col.parse().unwrap_or_default(),
                row: row.parse().unwrap_or_default(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{analyser::zig::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error() {
        static LOG: &str = include_str!("../../tests/zig_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "expected type expression, found ')'".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/exercises/011_while.zig".to_string(),
                    row: 24,
                    col: 15
                }]
            }]
        );
    }

    #[test]
    fn note() {
        static LOG: &str = include_str!("../../tests/zig_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "all non-void values must be used".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/exercises/012_while2.zig".to_string(),
                        row: 28,
                        col: 27
                    }]
                },
                types::Message {
                    error: "this error can be suppressed by assigning the value to '_'".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/exercises/012_while2.zig".to_string(),
                        row: 28,
                        col: 27
                    }]
                }
            ]
        );
    }
}
