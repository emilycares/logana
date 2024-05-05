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
            if let Some(error) = parse_line(line, project_dir) {
                errors.push(error);
            }
        }
    }

    errors
}

fn parse_line(line: &str, project_dir: &str) -> Option<types::Message> {
    let mut line = line.splitn(4, ':');
    let file = line.next()?;
    let row = line.next()?;
    let col = line.next()?;
    let message = line.next()?.trim();

    let location = types::Location {
        path: format!("{project_dir}/src/{file}"),
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
    use crate::{analyser::alire::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error_1() {
        static LOG: &str = include_str!("../../tests/alire_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "missing string quote".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/buildbuddy.adb".to_string(),
                    row: 5,
                    col: 31
                }]
            },]
        );
    }
}
