use std::path::Path;

use subprocess::{Exec, Redirection};

use crate::types;

#[must_use]
pub fn analyse(log: &str, project_dir: &str, package: &str) -> types::AnalyseReport {
    let log = log.lines().collect();
    let errors = get_exceptions(&log, project_dir, package);

    types::AnalyseReport { errors }
}

/// Will return the package for a file
///
/// To Archive this we cut of the
/// - file with exetension
/// - `project_dir`
/// - "/src/java/main"
/// and replace slash with dots
#[must_use]
pub fn get_package(file: &str, project_dir: &str) -> Option<String> {
    let file = Path::new(file);
    let file_name = file.file_name().expect("The file that was supplied is not a file");
    let file_name = file_name.to_str().expect("The file that was supplied contains ileagal characters in its name");
    let file_related = "/".to_owned() + file_name;
    let file = file.to_str().expect("The file that was supplied contains ileagal characters in its path");
    Some(file.replace(project_dir, "")
        .replace("src/main/java/", "")
        .replace(&file_related, "")
        .replace('/', "."))
}

/// Will return the file for a class
#[must_use]
pub fn get_file(class: &str, project_dir: &str) -> String {
    let inter = class.replace('.', "/");
    format!("{project_dir}/src/main/java/{inter}.java")
}

#[must_use]
pub fn get_project_files(project_dir: &str) -> Vec<String> {
    let out = Exec::cmd("find")
        .arg(project_dir)
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg("*.java")
        .stdout(Redirection::Pipe)
        .capture()
        .expect("To get output")
        .stdout_str();

    let out: Vec<String> = out.lines().map(std::string::ToString::to_string).collect();

    out
}

#[must_use]
pub fn remove_function(path: &str) -> &str {
    let dots = path
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '.')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let last_dont_index = dots.last().expect("A package should contain dots");

    &path[0..*last_dont_index]
}

#[must_use]
pub fn get_row(row: &str) -> Option<usize> {
    let row = &row[0..row.len() - 1];

    let Some((_, row)) = row.split_once(':') else {
        return None;
    };

    let row = row.parse::<usize>().unwrap_or_default();

    Some(row)
}

#[must_use]
pub fn parse_exception(log: &[&str], project_dir: &str, package: &str) -> Option<types::Message> {
    let first_line = log.first().expect("An exeption log should contain lines");
    let Some((_, error)) = first_line.split_once(": ") else {
        return None;
    };

    let mut locations = vec![];

    'locations: for i in 1.. {
        let Some(line) = log.get(i) else {

            break 'locations;
        };

        let line = line.trim();
        let line: &str = &line[3..line.len()];

        if !line.starts_with(package) {
            continue;
        }

        let Some((path, row)) = line.split_once('(') else {
            continue;
        };

        let path = get_file(remove_function(path), project_dir);

        // When there is no row then it is not in source
        if let Some(row) = get_row(row) {
            let location = types::Location { path, row, col: 0 };
            locations.push(location);
        }
    }

    Some(types::Message {
        error: error.to_string(),
        locations,
    })
}

#[must_use]
pub fn get_exceptions(log: &Vec<&str>, project_dir: &str, package: &str) -> Vec<types::Message> {
    let mut errors = vec![];
    'log: for i in 1.. {
        let Some(line) = log.get(i) else {
            break 'log;
        };

        let line = line.trim();

        if (line.contains("Error: ") || line.contains("Exception: "))
            && !line.starts_with("Caused by:")
        {
            let mut end = 0;
            'exception: for y in 1.. {
                let y = i + y;
                let Some(line) = log.get(y) else {
                    break 'exception;
                };

                if !line.trim().starts_with("at ") {
                    if let Some(line) = log.get(y + 1) {
                        if !line.trim().starts_with("at ") {
                            break 'exception;
                        }
                    };
                }

                end = y;
            }

            let end = end + 1;
            let exception_log = &log[i..end];
            if let Some(error) = parse_exception(exception_log, project_dir, package) {
                errors.push(error);
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::java::{analyse, get_file, get_package, parse_exception},
        types,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn should_find_build_error() {
        static LOG: &str = include_str!("../../tests/java_1.log");
        let result = analyse(LOG, "/tmp/project", "my.rootpackage.name");

        assert_eq!( result, types::AnalyseReport {
            errors: vec![
                types::Message {
                    error: "java.lang.NullPointerException: Cannot invoke \"String.split(String)\" because \"abc\" is null".to_string(),
                    locations: vec![
                        types::Location {
                            path: "/tmp/project/src/main/java/my/rootpackage/name/AbcController.java".to_string(),
                            row: 21,
                            col: 0
                        },
                        types::Location {
                            path: "/tmp/project/src/main/java/my/rootpackage/name/MyLibrary.java".to_string(),
                            row: 44,
                            col: 0,
                        }
                    ]
                },
                types::Message {
                    error: "1 expectation failed.       ".to_string(),
                    locations: vec![
                        types::Location {
                            path: "/tmp/project/src/main/java/my/rootpackage/name/MyLibraryTest.java".to_string(),
                            row: 32,
                            col: 0,
                        }
                    ]
                }
            ]
        });
    }

    #[test]
    fn get_package_test() {
        let project_dir = "/tmp/project/";
        let file = "/tmp/project/src/main/java/my/rootpackage/name/MyLibrary.java";
        assert_eq!(
            Some("my.rootpackage.name".to_string()),
            get_package(file, project_dir)
        );
    }

    #[test]
    fn get_file_test() {
        let project_dir = "/tmp/project";
        let package = "my.rootpackage.name.AbcController";
        assert_eq!(
            "/tmp/project/src/main/java/my/rootpackage/name/AbcController.java".to_string(),
            get_file(package, project_dir)
        );
    }

    #[test]
    fn should_parse_exception_1() {
        static LOG: &str = include_str!("../../tests/java_exeption_1.log");
        let log: Vec<&str> = LOG.lines().collect();
        let result = parse_exception(&log, "/tmp/project", "my.rootpackage.name");
        assert_eq!(result, Some(types::Message {
            error: "java.lang.NullPointerException: Cannot invoke \"String.split(String)\" because \"abc\" is null".to_string(),
            locations: vec![types::Location {
                path: "/tmp/project/src/main/java/my/rootpackage/name/AbcController.java".to_string(),
                row: 21,
                col: 0,
            },
            types::Location {
                path: "/tmp/project/src/main/java/my/rootpackage/name/MyLibrary.java".to_string(),
                row: 44,
                col: 0
            }]
        }));
    }
}
