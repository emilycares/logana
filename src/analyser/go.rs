use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Go`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if let Some(err) = parse_line_error(line, project_dir) {
                errors.push(err);
            }
        }
    }

    errors
}

fn parse_line_error(line: &str, project_dir: &str) -> Option<types::Message> {
    let mut splits = line.splitn(4, ':').into_iter();
    let mut file = splits.next()?;
    if file.starts_with("./") {
        file = &file[2..];
    }
    let row = splits.next()?;
    let col = splits.next()?;
    let message = splits.next()?.trim();

    let location = types::Location {
        path: format!("{project_dir}/{file}"),
        col: col.parse().unwrap_or_default(),
        row: row.parse().unwrap_or_default(),
    };

    Some(types::Message {
        error: message.to_string(),
        locations: vec![location],
    })
}

#[cfg(test)]
mod tests {
    use crate::{analyser::go::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_find_build_error_1() {
        static LOG: &str = include_str!("../../tests/go_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "string literal not terminated".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/main.go".to_string(),
                    row: 4,
                    col: 2
                }]
            }]
        );
    }

    #[test]
    fn should_find_build_error_2() {
        static LOG: &str = include_str!("../../tests/go_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "undefined: fmt.PrintLn".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/main.go".to_string(),
                    row: 8,
                    col: 6
                }]
            }]
        );
    }
}
