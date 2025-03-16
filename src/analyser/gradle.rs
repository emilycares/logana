use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Gradle`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];

    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();
    let mut current_module = None;

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            let line = line.trim();

            if line.starts_with("> Task :") {
                let line = line.trim_start_matches("> Task :");
                if let Some((module, _)) = line.split_once(":") {
                    current_module = Some(module);
                }
            }

            // Only works with "./gradlew test --info"
            if line.ends_with("FAILED") {
                parse_failed_test(i, line_len, lines, project_dir, current_module, &mut errors);
            }

            if line.starts_with(project_dir) {
                if let Some(error) = parse_error(line, lines.get(i + 2).copied()) {
                    errors.push(error);
                }
            }
        }
    }

    errors
}

fn parse_failed_test(
    i: usize,
    line_len: &usize,
    lines: &[&str],
    project_dir: &str,
    current_module: Option<&str>,
    errors: &mut Vec<types::Message>,
) {
    let mut msg = None;
    'test_case: for i in i..*line_len {
        if let Some(line) = lines.get(i) {
            if line.is_empty() {
                break 'test_case;
            }
            if line.contains("expected:") {
                if let Some(expected_line) = line.split_once("expected:") {
                    msg = Some(expected_line.1);
                }
            }
            if !line.contains("at ") {
                continue 'test_case;
            }
            if line.contains("org.junit.jupiter") {
                continue 'test_case;
            }
            if let Some((_, line)) = line.split_once("app//") {
                if let Some((path, rest)) = line.split_once("(") {
                    let mut language = "java";
                    if let Some((_, new_lang)) = rest.split_once(".") {
                        if let Some((new_lang, _)) = new_lang.rsplit_once(":") {
                            language = new_lang;
                        }
                    }

                    if let Some((filename, line)) = rest.split_once(&format!(".{language}:")) {
                        let line_number = line.trim_end_matches(")");

                        let language_identifier = match language {
                            "java" => "java",
                            "kt" => "kotlin",
                            a => a,
                        };
                        if let Some((class_path, _)) = path.split_once(filename) {
                            let module = match current_module {
                                None => "",
                                Some(module) => &format!("/{module}"),
                            };
                            let path = format!(
                                "{}{}/src/test/{}/{}{}.{}",
                                project_dir,
                                module,
                                language_identifier,
                                class_path.replace(".", "/"),
                                filename,
                                language
                            );
                            if let Some(error) = msg {
                                errors.push(types::Message {
                                    error: error.trim().to_owned(),
                                    locations: vec![types::Location {
                                        path: path.to_string(),
                                        row: line_number.parse::<usize>().unwrap_or_default(),
                                        col: 0,
                                    }],
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse_error(line: &str, col_line: Option<&str>) -> Option<types::Message> {
    let mut split = line.split(':');

    let path = split.next()?;
    let row = split.next()?;
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
    use crate::{analyser::gradle::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_find_syntax_error() {
        static LOG: &str = include_str!("../../tests/gradle_java_syntax.log");
        let result = analyse(LOG, "/home/emily/tmp/gradle-test");

        assert_eq!(
            result,
            vec![types::Message {
                error: "error ';' expected".to_string(),
                locations: vec![types::Location {
                    path: "/home/emily/tmp/gradle-test/app/src/main/java/gradle/test/App.java"
                        .to_string(),
                    row: 8,
                    col: 30
                }]
            }]
        );
    }

    #[test]
    fn should_find_test_error() {
        static LOG: &str = include_str!("../../tests/gradle_test.log");
        let result = analyse(LOG, "/home/emily/tmp/gradle-test");

        assert_eq!(
            result,
            vec![types::Message {
                error: "not <null>".to_string(),
                locations: vec![types::Location {
                    path: "/home/emily/tmp/gradle-test/src/test/java/org/example/AppTest.java"
                        .to_string(),
                    row: 13,
                    col: 0
                }]
            }]
        );
    }

    #[test]
    fn should_find_test_error_in_kotlin() {
        static LOG: &str = include_str!("../../tests/gradle_kotlin.log");
        let result = analyse(LOG, "/home/emily/tmp/gradle-test");

        assert_eq!(
            result,
            vec![types::Message {
                error: "<true> but was: <false>".to_string(),
                locations: vec![types::Location {
                    path:
                        "/home/emily/tmp/gradle-test/common/src/test/kotlin/org/example/AppTest.kt"
                            .to_string(),
                    row: 14,
                    col: 0
                }]
            }]
        );
    }
}
