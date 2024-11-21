use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Angular`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.starts_with("Build at:") {
                continue;
            }
            if let Some(error) = parse_line(line, project_dir) {
                errors.push(error);
            }
            if let Some(error) = parse_syntax_error(line) {
                errors.push(error);
            }
        }
    }

    errors
}

fn parse_syntax_error(line: &str) -> Option<types::Message> {
    let mut line = line;
    if line.starts_with("SyntaxError: ") {
        line = &line[13..];
    } else {
        return None;
    }
    let (path, rest) = line.split_once(": ")?;
    let (message, rest) = rest.split_once(" (")?;
    let (row, col) = rest.split_once(':')?;
    let mut col = col;
    col = &col[..col.len() - 1];

    let location = types::Location {
        path: path.to_string(),
        row: row.parse().unwrap_or_default(),
        col: col.parse().unwrap_or_default(),
    };

    Some(types::Message {
        error: message.to_string(),
        locations: vec![location],
    })
}

fn parse_line(line: &str, project_dir: &str) -> Option<types::Message> {
    let mut line = line;
    if line.to_uppercase().starts_with("ERROR: ") {
        line = &line[7..];
    }
    let (loc, message) = line.split_once(" - ")?;

    let mut splits = loc.splitn(3, ':');

    let file = splits.next()?.trim();
    let row = splits.next()?.trim();
    let col = splits.next()?.trim();

    let location = types::Location {
        path: format!("{project_dir}/{file}"),
        row: row.parse().unwrap_or_default(),
        col: col.parse().unwrap_or_default(),
    };

    Some(types::Message {
        error: message.to_string(),
        locations: vec![location],
    })
}

#[cfg(test)]
mod tests {
    use crate::{analyser::angular::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error_1() {
        static LOG: &str = include_str!("../../tests/angular_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "error TS2307: Cannot find module '../../../response' or its corresponding type declarations.".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/file.ts".to_string(),
                    row: 8,
                    col: 26
                }]
            },
            types::Message {
                error: "error TS2339: Property 'showMsg' does not exist on type '{ type: string; }'.".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/file.ts".to_string(),
                    row: 27,
                    col: 27
                }]
            }]
        );
    }

    #[test]
    fn error_syntax_error() {
        static LOG: &str = include_str!("../../tests/angular_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "Unexpected token".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/app/app.component.ts".to_string(),
                        row: 37,
                        col: 20
                    }]
                },
                types::Message {
                    error: "error TS1109: Expression expected.".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/app/app.component.ts".to_string(),
                        row: 12,
                        col: 1
                    }]
                }
            ]
        );
    }

    #[test]
    fn error_no_hash() {
        static LOG: &str = include_str!("../../tests/angular_3.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "error TS1109: Expression expected.".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/app/app.component.ts".to_string(),
                    row: 12,
                    col: 1
                }]
            }]
        );
    }
}
