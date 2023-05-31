use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Eslint`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
           for err in parse_file_lint(i, line, lines, project_dir) {
               errors.push(err);
           }
        }
    }

    errors
}

/// input:
/// 1  /tmp/project/index.ts
///    --------------------- path
/// 2   1:1  error    Unexpected var, use let or const instead  no-var
///     ---  -------------------------------------------------
///     |    |
///     |    Error message
///     call parse_location                                  
/// 3   1:5  warning  'as' is assigned a value but never used   @typescript-eslint/no-unused-vars
/// |   ---  ------------------------------------------------
/// |   |    |
/// |   |    Error message
/// |   call parse_location                                  
/// |
/// line numbers
///
fn parse_file_lint(start_index: usize, line: &str, lines: &[&str], project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    if line.starts_with(project_dir) {
        let path = line;

        'errors: for y in 1.. {
            let y = start_index + y;
            let Some(line) = lines.get(y) else {
                        break 'errors;
                    };
            let line = line.trim();

            if line.is_empty() {
                break 'errors;
            }

            let split = line
                .split(' ')
                .into_iter()
                .filter(|l| !l.is_empty())
                .collect::<Vec<&str>>();

            let error = split[1..split.len() - 1].join(" ");

            if let Some(location) = parse_location(split[0], path) {
                errors.push(types::Message {
                    error,
                    locations: vec![location],
                });
            }
        }
    }

    return errors;
}

/// line_numbers -> 1:8
///                 | |
///                 | col
///                 row
fn parse_location(line_numbers: &str, path: &str) -> Option<types::Location> {
    if let Some((row, col)) = line_numbers.split_once(':') {
        return Some(types::Location {
            path: path.to_string(),
            col: col.parse().unwrap_or_default(),
            row: row.parse().unwrap_or_default(),
        });
    }

    None
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::{analyser::eslint::analyse, core::types};

    #[test]
    fn should_find_lint() {
        static LOG: &str = include_str!("../../tests/eslint_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "error Parsing error: ','".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/file.ts".to_string(),
                        row: 1,
                        col: 8
                    }]
                },
                types::Message {
                    error: "error Unexpected var, use let or const instead".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/index.ts".to_string(),
                        row: 1,
                        col: 1,
                    }]
                },
                types::Message {
                    error: "warning 'as' is assigned a value but never used".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/index.ts".to_string(),
                        row: 1,
                        col: 5,
                    }]
                }
            ]
        );
    }
}
