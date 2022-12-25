use crate::types;

pub fn analyse(log: &str, project_dir: &str) -> types::AnalyseReport {
    let mut errors: Vec<types::Message> = vec![];
    let lines: Vec<&str> = log.lines().collect();

    for i in 0..lines.len() {
        if let Some(line) = lines.get(i) {
            if line.starts_with("error: ")
                || line.starts_with("error[")
                || line.starts_with("warning: ")
            {
                if let Some((_, error)) = line.split_once(": ") {
                    if let Some(location_line) = lines.get(i + 1) {
                        let location_line = location_line.trim();
                        if location_line.starts_with("-->") {
                            let location = &location_line[4..];

                            if let Some(location) = parse_location(location, project_dir) {
                                errors.push(types::Message {
                                    error: error.to_string(),
                                    locations: vec![location],
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    types::AnalyseReport {
        errors
    }
}

fn parse_location(location: &str, project_dir: &str) -> Option<types::Location> {
    let parts: Vec<&str> = location.split(':').collect();

    if let Some(path) = parts.first() {
        let path = path.to_string();
        if let Some(row) = parts.get(1) {
            if let Ok(row) = row.parse::<usize>() {
                if let Some(col) = parts.get(2) {
                    if let Ok(col) = col.parse::<usize>() {
                        return Some(types::Location {
                            path: format!("{}/{}", project_dir, path),
                            row,
                            col,
                        });
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{analyser::cargo::analyse, types};

    #[test]
    fn should_find_clippy_error() {
        static LOG: &'static str = include_str!("../../tests/cargo_clippy_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![
                    types::Message {
                        error: "unused variable: `i`".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/split.rs".to_string(),
                            row: 9,
                            col: 19
                        }]
                    },
                    types::Message {
                        error: "unused variable: `last`".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/split.rs".to_string(),
                            row: 4,
                            col: 9
                        }]
                    },
                    types::Message {
                        error: "unused variable: `split_lines`".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/split.rs".to_string(),
                            row: 6,
                            col: 9
                        }]
                    },
                    types::Message {
                        error: "variable does not need to be mutable".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/split.rs".to_string(),
                            row: 2,
                            col: 9
                        }]
                    },
                    types::Message {
                        error: "function `get_pane_content` is never used".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/fetch.rs".to_string(),
                            row: 4,
                            col: 8
                        }]
                    },
                    types::Message {
                        error: "function `split_builds` is never used".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/loader/split.rs".to_string(),
                            row: 1,
                            col: 8
                        }]
                    },
                    types::Message {
                        error: "single-character string constant used as pattern".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/analyser/cargo.rs".to_string(),
                            row: 43,
                            col: 43
                        }]
                    },
                    types::Message {
                        error: "accessing first element with `parts.get(0)`".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/analyser/cargo.rs".to_string(),
                            row: 45,
                            col: 25
                        }]
                    },
                    types::Message {
                        error: "you are deriving `PartialEq` and can implement `Eq`".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/types.rs".to_string(),
                            row: 3,
                            col: 17
                        }]
                    }
                ]
            }
        )
    }

    #[test]
    fn should_find_build_error() {
        static LOG: &'static str = include_str!("../../tests/cargo_split_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                errors: vec![
                    types::Message {
                        error: "cannot find value `asd` in this scope".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/main.rs".to_string(),
                            row: 2,
                            col: 5
                        }]
                    },
                    types::Message {
                        error: "cannot find value `asd` in this scope".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/main.rs".to_string(),
                            row: 2,
                            col: 5
                        }]
                    },
                ]
            }
        )
    }
}
