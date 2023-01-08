use std::path::Path;

use crate::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Java`]
#[must_use]
pub fn analyse(lines: &[&str], project_dir: &str, package: &str) -> types::AnalyseReport {
    let errors = get_exceptions(lines, project_dir, package);

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

/// Returns all java files for project
//fn get_project_files(project_dir: &str) -> Vec<String> {
//let out = Exec::cmd("find")
//.arg(project_dir)
//.arg("-type")
//.arg("f")
//.arg("-name")
//.arg("*.java")
//.stdout(Redirection::Pipe)
//.capture()
//.expect("To get output")
//.stdout_str();

//let out: Vec<String> = out.lines().map(std::string::ToString::to_string).collect();

//out
//}

#[must_use]
fn remove_function(path: &str) -> &str {
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
fn get_row(row: &str) -> Option<usize> {
    let row = &row[0..row.len() - 1];

    let Some((_, row)) = row.split_once(':') else {
        return None;
    };

    let row = row.parse::<usize>().unwrap_or_default();

    Some(row)
}

#[must_use]
fn parse_exception(log: &[&str], project_dir: &str, package: &str) -> Option<types::Message> {
    let first_line = log.first().expect("An exception log should contain lines");
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
fn get_exceptions(log: &[&str], project_dir: &str, package: &str) -> Vec<types::Message> {
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

            if end != 0 {
                let end = end + 1;
                let exception_log = &log[i..end];
                if let Some(error) = parse_exception(exception_log, project_dir, package) {
                    errors.push(error);
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use crate::{
        analyser::java::{analyse, get_project_files, get_project_package},
        types,
    };

    //#[test]
    //fn should_find_build_error() {
    //static LOG: &'static str = include_str!("../../tests/java_exeption_1.log");
    //let result = analyse(LOG, "/tmp/project");

    //assert_eq!(
    //result,
    //types::AnalyseReport {
    //errors: vec![
    //types::Message {
    //error: "org.jboss.resteasy.spi.UnhandledException: java.lang.NullPointerException: Cannot invoke \"String.split(String)\" because \"abc\" is null".to_string(),
    //locations: vec![types::Location {
    //path: "/tmp/project/src/main/java/my/rootpackage/name/AbcController.java".to_string(),
    //row: 21,
    //col: 0
    //}]
    //}
    //]
    //}
    //)
    //}

    #[test]
    fn get_project_files_test() {
        assert_eq!(
            vec![
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/Main.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/view/RemoveInventory.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/view/LocationInventory.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/view/ConfigInventory.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/view/DeployedMapsInventory.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigDeployCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigSubCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigValidateCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigGuiCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigDeployedMapsCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigGenerateCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigRewardCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigMapCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigNoValidationSubCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/map/ConfigMapDeleteCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/config/ConfigImportsMapsCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/command/ConfigCommand.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/mover/SignRemover.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/repository/MapRepository.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/SignType.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/MapValidator.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/generator/IConfigurationGenerator.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/generator/YamlConfigurationGenerator.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/supplier/PlotMeRegionSupplier.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/supplier/IRectRegionSupplier.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/MapMover.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/scanner/ScannerFactory.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/scanner/IScanner.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/scanner/ScheduledExecutorService.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/scanner/AsyncScanner.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/scanner/DefaultScanner.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/SignAnalyzer.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/MapDeployer.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/controller/RewardController.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/IDataPoint.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/RectRegion.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/datapoint/PlayerSpawn.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/datapoint/MapInformation.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/datapoint/MiddleSpawn.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/datapoint/ItemSpawn.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/map/UnscannableMap.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/map/Map.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/impl/MapType.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/ValidationResult.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/model/PermissionType.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/PlotExeption.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/Icon.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/storage/model/DeployedMapPosition.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/storage/model/DeployedMap.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/storage/impl/FileDeploymentStorageImpl.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/util/storage/DeploymentStorageIfc.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/event/BlockPlaceListener.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/event/WeatherChangeListener.java",
"/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/event/PlayerInteractListener.java",
"/home/michael/Documents/projects/smash/config/src/test/java/eu/smashmc/config/controller/generator/YamlConfigurationGeneratorTest.java",
"/home/michael/Documents/projects/smash/config/src/test/java/eu/smashmc/config/controller/SignAnalyzerTest.java",
"/home/michael/Documents/projects/smash/config/src/test/java/eu/smashmc/config/controller/MapMoverTest.java",
"/home/michael/Documents/projects/smash/config/src/test/java/eu/smashmc/config/util/storage/impl/FileDeploymentStorageImplTest.java"
        ],
            get_project_files("/home/michael/Documents/projects/smash/config")
        );
    }

    #[test]
    fn get_project_package_test() {
        let project_dir = "/home/michael/Documents/projects/smash/config/";
        let file = "/home/michael/Documents/projects/smash/config/src/main/java/eu/smashmc/config/Main.java";
        assert_eq!(
            "eu.smashmc.config".to_string(),
            get_project_package(file, project_dir)
        );
    }
}
