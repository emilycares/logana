use std::path::Path;

use subprocess::{Exec, Redirection};

use crate::types;

pub fn analyse(log: &str, project_dir: &str) -> types::AnalyseReport {
    let mut errors: Vec<types::Message> = vec![];
    let lines: Vec<&str> = log.lines().collect();

    for i in 0..lines.len() {
        if let Some(line) = lines.get(i) {
            if line.contains("Exeption: ") {
                let error_message = line;
            }
        }
    }
    types::AnalyseReport { errors }
}

/// Will retrn the projets package
///
/// To Archive this we cut of the
/// - file with exetension
/// - project_dir
/// - "/src/java/main"
/// and replace slash with dots
pub fn get_project_package(lowest_file: &str, project_dir: &str) -> String {
    let lowest_file = Path::new(lowest_file);
    let file_name = lowest_file.file_name().unwrap().to_str().unwrap();
    let file_related = "/".to_owned() + file_name;
    let lowest_file = lowest_file.to_str().unwrap();
    lowest_file
        .replace(project_dir, "")
        .replace("src/main/java/", "")
        .replace(&file_related, "")
        .replace('/', ".")
}

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

    let out: Vec<String> = out.lines().map(|s| s.to_string()).collect();

    out
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::java::{analyse, get_project_files, get_project_package},
        types,
    };

    #[test]
    fn should_find_build_error() {
        static LOG: &'static str = include_str!("../../tests/java_exeption_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!( result, types::AnalyseReport { 
            errors: vec![ types::Message { 
                error: "org.jboss.resteasy.spi.UnhandledException: java.lang.NullPointerException: Cannot invoke \"String.split(String)\" because \"abc\" is null".to_string(), 
                locations: vec![types::Location { 
                    path: "/tmp/project/src/main/java/my/rootpackage/name/AbcController.java".to_string(),
                    row: 21,
                    col: 0 
                }]
            }] 
        })
    }

    #[test]
    fn get_project_package_test() {
        let project_dir = "/tmp/project";
        let file = "/tmp/project/src/main/java/my/rootpackage/name/MyLibrary.java";
        assert_eq!(
            "my.rootpackage.name".to_string(),
            get_project_package(file, project_dir)
        );
    }
}
