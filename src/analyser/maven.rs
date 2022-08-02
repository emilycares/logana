use std::fmt::Display;

use crate::types;

pub fn analyse(log: &str, project_dir: &str) -> MavenAnalyseReport {
    let mut copiler_errors: Vec<MavenMessage> = vec![];
    let mut test_failures: Vec<MavenMessage> = vec![];
    let mut phase = MavenPhase::Scanning;

    let lines: Vec<&str> = log.lines().collect();

    for i in 0..lines.len() {
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
                MavenPhase::Scanning => {}
                MavenPhase::Building => {
                    let begining = format!("[ERROR] {}", project_dir);
                    if line.starts_with(&begining) {
                        if let Some(message) = parse_copilation_error(line) {
                            copiler_errors.push(message);
                        }
                    }
                }
                MavenPhase::Testing => {
                    if line.starts_with("org.opentest4j.AssertionFailedError") {
                        if let Some((_, error)) = line.split_once(':') {
                            let error = &error[1..];
                            for y in 1.. {
                                let i = i + y;
                                if let Some(line) = lines.get(i) {
                                    if !line.starts_with("\tat ") {
                                        break;
                                    }
                                    let line = &line[4..];
                                    if let Some(location) = parse_test_location(line, project_dir) {
                                        test_failures.push(MavenMessage {
                                            error: error.to_string(),
                                            locations: vec![location],
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
                MavenPhase::Done => {}
            }
        }
    }

    MavenAnalyseReport {
        copiler_errors,
        test_failures,
    }
}

fn parse_copilation_error(error: &str) -> Option<MavenMessage> {
    // remove "[ERROR] "
    let error = &error[8..];

    if let Some((location, _)) = error.split_once(' ') {
        if let Some(location) = parse_coppilation_location(location) {
            if let Some((_, error)) = error.split_once("] ") {
                return Some(MavenMessage {
                    error: error.to_string(),
                    locations: vec![location],
                });
            }
        }
    }

    None
}

fn parse_coppilation_location(location: &str) -> Option<types::Location> {
    if let Some((path, row_col)) = location.split_once(':') {
        let row_col = &row_col[1..];
        let row_col = &row_col[..row_col.len() - 1];

        if let Some((row, col)) = row_col.split_once(',') {
            return Some(types::Location {
                path: path.to_string(),
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
                let path = format!(
                    "{}/src/test/java/{}{}.java",
                    project_dir, class_path, class_name
                );

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

#[derive(Debug, PartialEq)]
pub struct MavenAnalyseReport {
    pub copiler_errors: Vec<MavenMessage>,
    pub test_failures: Vec<MavenMessage>,
}

impl MavenAnalyseReport {
    pub fn new() -> Self {
        Self {
            copiler_errors: vec![],
            test_failures: vec![],
        }
    }
}

impl Display for MavenAnalyseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let test_failures = self.test_failures.iter();
        self.copiler_errors
            .iter()
            .chain(test_failures)
            .fold(Ok(()), |result, message| {
                result.and_then(|_| writeln!(f, "{}", message))
            })
    }
}

impl Default for MavenAnalyseReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct MavenMessage {
    pub error: String,
    pub locations: Vec<types::Location>,
}

impl Display for MavenMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = self.locations.first() {
            write!(f, "{}|{}", location, self.error)
        } else {
            write!(f, "")
        }
    }
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
        analyser::maven::{analyse, parse_test_location, MavenAnalyseReport, MavenMessage},
        types,
    };

    #[test]
    fn should_find_sytax_error() {
        static LOG: &'static str = include_str!("../../tests/maven_copilation_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            MavenAnalyseReport {
                copiler_errors: vec![MavenMessage {
                    error: "';' expected".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                        row: 18,
                        col: 54
                    }]
                }],
                test_failures: vec![]
            }
        )
    }

    #[test]
    fn should_find_unknown_symbol() {
        static LOG: &'static str = include_str!("../../tests/maven_copilation_2.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            MavenAnalyseReport {
                copiler_errors: vec![MavenMessage {
                    error: "cannot find symbol".to_string(),
                    locations: vec![types::Location {
                        path: "/tmp/project/src/main/java/some/thing/project/Main.java".to_string(),
                        row: 45,
                        col: 4
                    }]
                }],
                test_failures: vec![]
            }
        )
    }

    #[test]
    fn should_find_failed_test() {
        static LOG: &'static str = include_str!("../../tests/maven_test_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            MavenAnalyseReport {
                copiler_errors: vec![],
                test_failures: vec![
                    MavenMessage {
                        error: "expected: <true> but was: <false>".to_string(),
                        locations: vec![
                            types::Location {
                                path: "/tmp/project/src/test/java/some/thing/project/controller/AnalyzerTest.java".to_string(),
                                row: 34,
                                col:  0
                            }
                        ]
                    },
                    MavenMessage {
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
        )
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
        )
    }
}
