use crate::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Maven`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> types::AnalyseReport {
    let mut errors: Vec<types::Message> = vec![];

    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            let line = line.trim();

            if line.starts_with(project_dir) {
                if let Some(error) = parse_error(line, lines.get(i + 2).copied()) {
                    errors.push(error);
                }
            }
        }
    }

    types::AnalyseReport { errors }
}

fn parse_error(line: &str, col_line: Option<&str>) -> Option<types::Message> {
    let mut split = line.split(':');

    let Some(path) = split.next() else {
        return None;
    };
    let Some(row) = split.next() else {
        return None;
    };
    let Ok(row) = row.parse::<usize>() else {
        return None;
    };

    let mut message = String::new();

    'message: loop {
        let Some(msg) = split.next() else {
            break 'message;
        };
        message += msg;
    }

    if let Some(col_line) = col_line {
        if let Some(col) = col_line.find('^') {
            return Some(types::Message {
                error: message.trim().to_string(),
                locations: vec![types::Location {
                    path: path.to_string(),
                    row,
                    col: col + 1,
                }],
            });
        }
    }
    Some(types::Message {
        error: message,
        locations: vec![types::Location {
            path: path.to_string(),
            row,
            col: 0,
        }],
    })
}

#[cfg(test)]
mod tests {
    use crate::{analyser::gradle::analyse, types};

    #[test]
    fn should_find_syntax_error() {
        static LOG: &str = include_str!("../../tests/gradle_java_syntax.log");
        let result = analyse(LOG, "/home/michael/tmp/gradle-test");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![types::Message {
                    error: "error ';' expected".to_string(),
                    locations: vec![types::Location {
                        path:
                            "/home/michael/tmp/gradle-test/app/src/main/java/gradle/test/App.java"
                                .to_string(),
                        row: 8,
                        col: 30
                    }]
                }]
            }
        );
    }
}
