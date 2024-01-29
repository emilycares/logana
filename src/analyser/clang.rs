use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Clang`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.starts_with(project_dir) || line.starts_with("src/") {
                if let Some(error) = parse_line(line, project_dir) {
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
/// "/tmp/project/main.c:47:5: warning: this style of line directive is a GNU extension [-Wgnu-line-marker]"
///  ------------------------           -------------------------------------------------------------------
///  `parse_location`()                   message
fn parse_line(line: &str, project_dir: &str) -> Option<types::Message> {
    if let Some(location) = parse_location(line, project_dir) {
        let message = line.splitn(4, ':').nth(3)?;
        return Some(types::Message {
            error: message.trim_start().to_string(),
            locations: vec![location],
        });
    }

    None
}

/// Parses location
///
/// Gets location input like:
/// "/tmp/project/main.c:47:5: warning: this style of line directive is a GNU extension [-Wgnu-line-marker]"
///  ------------------- -- -
///  path                |  |
///                      |  col
///                      row
fn parse_location(location: &str, project_dir: &str) -> Option<types::Location> {
    let mut spl = location.splitn(4, ':');
    let mut path = spl.next()?.to_string();
    // To full path if not already the case
    if !path.starts_with('/') {
        path = format!("{project_dir}/{path}");
    }

    let row = spl.next()?;
    let col = spl.next()?;
    Some(types::Location {
        path,
        col: col.parse().unwrap_or_default(),
        row: row.parse().unwrap_or_default(),
    })
}
#[cfg(test)]
mod tests {
    use crate::{analyser::clang::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error() {
        static LOG: &str = include_str!("../../tests/clang_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error: "warning: this style of line directive is a GNU extension [-Wgnu-line-marker]".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.c".to_string(),
                        row: 47,
                        col: 5
                    }]
                },
                types::Message {
                    error: "error: type specifier missing, defaults to 'int'; ISO C99 and later do not support implicit int [-Wimplicit-int]".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/main.c".to_string(),
                        row: 48,
                        col: 14
                    }]
                }
            ]
        );
    }

    #[test]
    fn error_version_14() {
        static LOG: &str = include_str!("../../tests/clang_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                types::Message {
                    error:
                        "warning: 'always_inline' function might not be inlinable [-Wattributes]"
                            .to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/string_map.cpp".to_string(),
                        row: 148,
                        col: 16
                    }]
                },
                types::Message {
                    error:
                        "warning: 'always_inline' function might not be inlinable [-Wattributes]"
                            .to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/string_map.cpp".to_string(),
                        row: 148,
                        col: 16
                    }]
                }
            ]
        );
    }
}
