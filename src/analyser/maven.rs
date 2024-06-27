use crate::core::types;
use itertools::Itertools;

/// Contains the analyser code for the [`crate::config::ParserKind::Maven`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];

    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            let beginning = format!("[ERROR] {project_dir}");
            if line.starts_with(&beginning) {
                if let Some(message) = parse_copilation_error(line) {
                    errors.push(message);
                }
            }

            if line.contains("<<< FAILURE!") {
                if let Some(message) = parse_test_exception(i, lines, project_dir) {
                    errors.push(message);
                }
            }
        }
    }

    // In maven some errors ar duplicated.
    errors.into_iter().unique().collect()
}

/// "[ERROR] /tmp/project/src/main/java/some/thing/project/Main.java:[45,4] cannot find symbol"
///  ------ --------------------------------------------------------------  -----------------
///  |      `parse_coppilation_location`()                                    message
///  cut away
fn parse_copilation_error(error: &str) -> Option<types::Message> {
    if let Some(error) = error.strip_prefix("[ERROR] ") {
        if let Some((location, _)) = error.split_once(' ') {
            if let Some(location) = parse_coppilation_location(location) {
                if let Some((_, error)) = error.split_once("] ") {
                    return Some(types::Message {
                        error: error.to_string(),
                        locations: vec![location],
                    });
                }
            }
        }
    }

    None
}

fn parse_test_exception(index: usize, lines: &[&str], project_dir: &str) -> Option<types::Message> {
    let mut message = String::new();
    for line in &lines[index + 1..lines.len()] {
        // Make sure to stop parsing at next failure
        if line.ends_with("<<< FAILURE!") {
            break;
        }

        let line = line.trim();
        if let Some(location) = line.strip_prefix("at ") {
            if location.starts_with("org.junit") {
                continue;
            }

            if let Some(location) = parse_test_location(location, project_dir) {
                return Some(types::Message {
                    error: message,
                    locations: vec![location],
                });
            }
            break;
        }
        if !line.starts_with("[ERROR] ") && !line.starts_with("-> at") {
            message.push_str(line);
        }
    }

    None
}

/// "/tmp/project/src/main/java/some/thing/project/Main.java:[18,54]"
///  -------------------------------------------------------  -- --
///  path                                                     |  col
///                                                           row
fn parse_coppilation_location(location: &str) -> Option<types::Location> {
    let mut location = location;

    // Handle possible drive letter
    let mut drive = "";
    if location.chars().nth(1) == Some(':') {
        drive = &location[0..2];
        location = &location[2..];
    }

    if let Some((path, row_col)) = location.split_once(':') {
        // remove braces
        let row_col = &row_col[1..row_col.len() - 1];

        if let Some((row, col)) = row_col.split_once(',') {
            return Some(types::Location {
                path: format!("{drive}{path}"),
                col: col.parse().unwrap_or_default(),
                row: row.parse().unwrap_or_default(),
            });
        }
    }

    None
}

/// "`some.thing.project.controller.AnalyzerTest.should_Test(AnalyzerTest.java:34`)"
fn parse_test_location(location: &str, project_dir: &str) -> Option<types::Location> {
    if let Some(class_name) = parse_class_name_from_test_location(location) {
        if let Some((class_path, _)) = location.split_once(class_name) {
            let class_path = class_path.replace('.', "/");
            if let Some(row) = parse_row_from_test_location(location) {
                let path = format!("{project_dir}/src/test/java/{class_path}{class_name}.java");

                return Some(types::Location {
                    path,
                    row: row.to_owned(),
                    col: 0,
                });
            }
        }
    }
    None
}

/// "`some.thing.project.controller.AnalyzerTest.should_Test(AnalyzerTest.java:34`)"
///                                                         ------------
///                                                         result
fn parse_class_name_from_test_location(location: &str) -> Option<&str> {
    if let Some((_, includes_class_name)) = location.split_once('(') {
        if let Some((class_name, _)) = includes_class_name.split_once('.') {
            return Some(class_name);
        }
    }

    None
}

/// "`some.thing.project.controller.AnalyzerTest.should_Testasd(AnalyzerTest.java:39`)
///  --------------------------------------------------------------------------- | -
///  split away                                                                  | Remove last brace
///                                                                              Parse number
fn parse_row_from_test_location(location: &str) -> Option<usize> {
    if let Some((_, includes_row)) = location.split_once(':') {
        let row = &includes_row[..includes_row.len() - 1];
        let number = row.parse::<usize>().unwrap_or_default();
        return Some(number);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::maven::{analyse, parse_test_location},
        core::types,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_find_syntax_error() {
        static LOG: &str = include_str!("../../tests/maven_copilation_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "';' expected".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                    row: 18,
                    col: 54
                }]
            }]
        );
    }

    #[test]
    fn should_find_unknown_symbol() {
        static LOG: &str = include_str!("../../tests/maven_copilation_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "cannot find symbol".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                    row: 45,
                    col: 4
                }]
            }]
        );
    }

    #[test]
    fn should_find_expected_symbol_on_windows() {
        static LOG: &str = include_str!("../../tests/maven_copilation_3.log");
        let result = analyse(LOG, "C:\\Users\\michael\\testproject");

        assert_eq!(
            result,
             vec![types::Message {
                    error: "error: ';' expected".to_string(),
                    locations: vec![types::Location {
                        path: "C:\\Users\\michael\\testproject\\src\\main\\java\\com\\micmine\\test\\Service.java".to_string(),
                        row: 604,
                        col: 98
                    }]
                }]
        );
    }

    #[test]
    fn should_find_failed_test_1() {
        static LOG: &str = include_str!("../../tests/maven_test_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                    types::Message {
                        error: "org.opentest4j.AssertionFailedError: expected: <true> but was: <false>".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java".to_string(),
                                row: 34,
                                col:  0
                            }
                        ]
                    },
                    types::Message {
                        error: "org.opentest4j.AssertionFailedError: expected: <1> but was: <2>".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java".to_string(),
                                row: 39,
                                col:  0
                            }
                        ]
                    }
                ]
        );
    }

    #[test]
    fn should_find_failed_test_2() {
        static LOG: &str = include_str!("../../tests/maven_test_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "java.util.ConcurrentModificationException".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/test/java/sone/thing/project/ThingTest.java"
                        .to_string(),
                    row: 145,
                    col: 0
                }]
            }]
        );
    }

    #[test]
    fn should_find_failed_test_3() {
        static LOG: &str = include_str!("../../tests/maven_test_3.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "org.opentest4j.AssertionFailedError: expected: <a> but was: <>".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/test/java/some/project/thing/ThingTest.java"
                        .to_string(),
                    row: 21,
                    col: 0
                }]
            }]
        );
    }

    #[test]
    fn should_find_failed_test_mockito() {
        static LOG: &str = include_str!("../../tests/maven_test_mockito.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![
                    types::Message {
                        error: "org.mockito.exceptions.verification.WantedButNotInvoked:Wanted but not invoked:publisher.publish(EventDTO(user=123, source=swiss));Actually, there were zero interactions with this mock.".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/project/thing/ThingTest.java".to_string(),
                                row: 34,
                                col:  0
                            }
                        ]
                    },
                ]
        );
    }

    #[test]
    fn parse_test_location_test() {
        let location =
            "some.thing.project.controller.AnalyzerTest.should_Test(AnalyzerTest.java:34)";
        let project_dir = "/tmp/project";

        assert_eq!(
            parse_test_location(location, project_dir),
            Some(types::Location {
                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java"
                    .to_string(),
                row: 34,
                col: 0
            })
        );
    }

    #[test]
    fn should_find_test_exception() {
        static LOG: &str = include_str!("../../tests/maven_test_exception.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "java.util.ConcurrentModificationException".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/test/java/some/thing/project/ServiceTest.java"
                        .to_string(),
                    row: 145,
                    col: 0
                }]
            }]
        );
    }

    #[test]
    fn should_find_mockito_error() {
        static LOG: &str = include_str!("../../tests/maven_mockito.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "org.mockito.exceptions.verification.WantedButNotInvoked:Wanted but not invoked:channel.publish(null);Actually, there were zero interactions with this mock.".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/test/java/some/thing/project/ServiceTest.java"
                        .to_string(),
                    row: 34,
                    col: 0
                }]
            }]
        );
    }
}
