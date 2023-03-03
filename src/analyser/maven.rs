use crate::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Maven`]
#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> types::AnalyseReport {
    let mut errors: Vec<types::Message> = vec![];
    let mut phase = MavenPhase::Scanning;

    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.starts_with("[INFO] Building") {
                phase = MavenPhase::Building;
            } else if line.starts_with("[INFO]  T E S T S") {
                phase = MavenPhase::Testing;
            } else if line.starts_with("[INFO] BUILD SUCCESS")
                || line.starts_with("[INFO] BUILD FAILURE")
            {
                phase = MavenPhase::Done;
            }

            match phase {
                MavenPhase::Scanning | MavenPhase::Done => {}
                MavenPhase::Building => {
                    let beginning = format!("[ERROR] {project_dir}");
                    if line.starts_with(&beginning) {
                        if let Some(message) = parse_copilation_error(line) {
                            errors.push(message);
                        }
                    }
                }
                MavenPhase::Testing => {
                    if line.starts_with("org.opentest4j.AssertionFailedError")
                        || line.starts_with("java.lang.AssertionError")
                    {
                        if let Some((_, error)) = line.split_once(':') {
                            let error = &error[1..];
                            'er: for y in 1.. {
                                let i = i + y;
                                if let Some(line) = lines.get(i) {
                                    if !line.starts_with("\tat ") {
                                        break;
                                    }
                                    let line = &line[4..];
                                    if let Some(location) = parse_test_location(line, project_dir) {
                                        errors.push(types::Message {
                                            error: error.to_string(),
                                            locations: vec![location],
                                        });
                                        break 'er;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    types::AnalyseReport { errors }
}

fn parse_copilation_error(error: &str) -> Option<types::Message> {
    // remove "[ERROR] "
    let error = &error[8..];

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

    None
}

fn parse_coppilation_location(location: &str) -> Option<types::Location> {
    let mut location = location;
    let mut drive = "";
    if location.chars().nth(1) == Some(':') {
        drive = &location[0..2];
        location = &location[2..];
    }

    if let Some((path, row_col)) = location.split_once(':') {
        let row_col = &row_col[1..];
        let row_col = &row_col[..row_col.len() - 1];

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

fn parse_class_name_from_test_location(location: &str) -> Option<&str> {
    if let Some((_, includes_class_name)) = location.split_once('(') {
        if let Some((class_name, _)) = includes_class_name.split_once('.') {
            return Some(class_name);
        }
    }

    None
}

fn parse_row_from_test_location(location: &str) -> Option<usize> {
    if let Some((_, includes_row)) = location.split_once(':') {
        let row = &includes_row[..includes_row.len() - 1];
        let number = row.parse::<usize>().unwrap_or_default();
        return Some(number);
    }

    None
}

enum MavenPhase {
    Scanning,
    Building,
    Testing,
    Done,
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::maven::{analyse, parse_test_location},
        types,
    };

    #[test]
    fn should_find_syntax_error() {
        static LOG: &str = include_str!("../../tests/maven_copilation_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![types::Message {
                    error: "';' expected".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                        row: 18,
                        col: 54
                    }]
                }]
            }
        );
    }

    #[test]
    fn should_find_unknown_symbol() {
        static LOG: &str = include_str!("../../tests/maven_copilation_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![types::Message {
                    error: "cannot find symbol".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                        row: 45,
                        col: 4
                    }]
                }]
            }
        );
    }

    #[test]
    fn should_find_expected_symbol_on_windows() {
        static LOG: &str = include_str!("../../tests/maven_copilation_3.log");
        let result = analyse(LOG, "C:\\Users\\michael\\testproject");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![types::Message {
                    error: "error: ';' expected".to_string(),
                    locations: vec![types::Location {
                        path: "C:\\Users\\michael\\testproject\\src\\main\\java\\com\\micmine\\test\\Service.java".to_string(),
                        row: 604,
                        col: 98
                    }]
                }]
            }
        );
    }

    #[test]
    fn should_find_failed_test() {
        static LOG: &str = include_str!("../../tests/maven_test_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![
                    types::Message {
                        error: "expected: <true> but was: <false>".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java".to_string(),
                                row: 34,
                                col:  0
                            }
                        ]
                    },
                    types::Message {
                        error: "expected: <1> but was: <2>".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java".to_string(),
                                row: 39,
                                col:  0
                            }
                        ]
                    }
                ]
            }
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
}
