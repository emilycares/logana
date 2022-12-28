use crate::types;

pub fn analyse(log: &str, project_dir: &str) -> types::AnalyseReport {
    let mut test_failures: Vec<types::Message> = vec![];
    let lines: Vec<&str> = log.lines().collect();

    for i in 0..lines.len() {
        if let Some(line) = lines.get(i) {
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("Error: ") || line_trimmed.starts_with("Usage:") || line_trimmed.starts_with("TypeError:") {
                let mut exeption = vec![line_trimmed];
                'exeption: for y in 1.. {
                    let i: usize = i + y;
                    let Some(line) = lines.get(i) else {
                      break 'exeption;
                    };

                    let line = line.trim();

                    if !line.starts_with("at ") {
                      break 'exeption;
                    }

                    exeption.push(line);
                }
                if let Some(message) = parse_exeption(exeption, project_dir) {
                    test_failures.push(message);
                }
            }
            if line_trimmed.starts_with("TypeError: ") {}
            if line.ends_with(" FAILED[39m") {
                if let Some(error) = lines.get(i + 1) {
                    let error = error.trim();

                    for y in 2.. {
                        let i = i + y;
                        if let Some(line) = lines.get(i) {
                            let line_trimmed = line.trim();
                            if !line_trimmed.starts_with("at ") {
                                break;
                            }

                            if line_trimmed.contains("(src/app") {
                                if let Some(location) =
                                    parse_test_location(line_trimmed, project_dir)
                                {
                                    test_failures.push(types::Message {
                                        error: error.to_string(),
                                        locations: vec![location],
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    types::AnalyseReport {
        compiler_errors: vec![],
        test_failures,
    }
}

pub fn parse_exeption(log: Vec<&str>, project_dir: &str) -> Option<types::Message> {
    let first_line = log.get(0).unwrap();
    let Some((_, error)) = first_line.split_once(": ") else {
        return None;
    };

    let mut locations = vec![];

    'locations: for i in 1.. {
        let Some(line) = log.get(i) else {
                  break 'locations;
                };

        // without closing bracket
        let line: &str = &line[1..line.len() - 1];

        let mut location = "".to_owned();

        if let Some((_, location_w)) = line.split_once("_karma_webpack_/webpack:") {
            location = location_w.to_owned();
        }

        if let Some((_, location_w)) = line.split_once(" (") {
            if location_w.starts_with("src/") {
                location = location_w.to_owned();
            }
        }

        //// without closing bracket
        //let location: &str = &location[1..location.len() - 1];

        if !location.starts_with("/") {
            location = "/".to_owned() + &location;
        }

        if !location.starts_with("/src/") {
            continue;
        }

        let Some((path, row_col)) = location.split_once(':') else {
                continue;
            };
        let path = format!("{}{}", project_dir, path);

        let Some((row, col)) = row_col.split_once(':') else  {
                    continue;
                };

        let row = row.parse::<usize>().unwrap_or_default();
        let col = col.parse::<usize>().unwrap_or_default();

        locations.push(types::Location { path, row, col });
    }

    Some(types::Message {
        error: error.to_string(),
        locations,
    })
}

fn parse_test_location(location: &str, project_dir: &str) -> Option<types::Location> {
    if let Some((_, location)) = location.split_once('(') {
        if let Some((path, row_col)) = location.split_once(':') {
            let path = format!("{}/{}", project_dir, path);

            let row_col = &row_col[..row_col.len() - 1];
            if let Some((row, col)) = row_col.split_once(':') {
                let row = row.parse::<usize>().unwrap_or_default();
                let col = col.parse::<usize>().unwrap_or_default();

                return Some(types::Location { path, row, col });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::karma_jasmine::{analyse, parse_exeption},
        types,
    };

    #[test]
    fn should_find_syntax_error() {
        static LOG: &'static str = include_str!("../../tests/karma_jasmine_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            types::AnalyseReport {
                compiler_errors: vec![],
                test_failures: vec![
                    types::Message {
                        error: "Expected true to be false.".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/app/app.component.spec.ts".to_string(),
                            row: 35,
                            col: 18
                        }]
                    },
                    types::Message {
                        error: "Expected OtherServiceService({  }) to be false.".to_string(),
                        locations: vec![types::Location {
                            path: "/tmp/project/src/app/components/other-service.service.spec.ts"
                                .to_string(),
                            row: 14,
                            col: 21
                        }]
                    }
                ],
            }
        )
    }

    #[test]
    fn should_parse_exeption_1() {
        static LOG: &'static str = include_str!("../../tests/karma_jasmine_exeption_1.log");
        let result = parse_exeption(LOG.lines().collect(), "/tmp/project");
        assert_eq!(result, Some(types::Message {
            error: "Cannot read property 'component' of undefined".to_string(), 
            locations: vec![types::Location {
                path: "/tmp/project/src/app/components/layout/main/command-info-dialog-modal/command-info-dialog-modal.component.ts".to_string(),
                row: 83,
                col: 1
            }]})
        );
    }

    #[test]
    fn should_parse_exeption_2() {
        static LOG: &'static str = include_str!("../../tests/karma_jasmine_exeption_2.log");
        let result = parse_exeption(LOG.lines().collect(), "/tmp/project");

        assert_eq!(result, Some(types::Message {
            error: "Expected '12.08.2021 08:01:06' to equal '12.08.2021 09:01:06'.".to_string(), 
            locations: vec![types::Location {
                path: "/tmp/project/src/app/components/layout/main/alarm-info-dialog-modal/functions/alarm-info-calculated-fields.functions.spec.ts".to_string(),
                row: 80,
                col: 22
            }]})
        );
    }
}
