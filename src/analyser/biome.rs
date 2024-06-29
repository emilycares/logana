use crate::core::types;

/// Contains the analyser code for the [`crate::config::ParserKind::Biome`]
#[must_use]
pub fn analyse(log: &str, _project_dir: &str) -> Vec<types::Message> {
    let mut errors: Vec<types::Message> = vec![];
    let lines = log.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let line_len = &lines.len();

    for i in 0..*line_len {
        if let Some(line) = lines.get(i) {
            if line.ends_with("━━━━━━━━━━") {
                if let Some(desc) = lines.get(i + 2) {
                    if let Some(error) = parse_line(line, desc) {
                        errors.push(error);
                    }
                }
            }
        }
    }

    errors
}

fn parse_line(line: &str, desc: &str) -> Option<types::Message> {
    let (loc, _) = line.split_once(" ")?;

    let mut splits = loc.split(':');
    let mut file: String = splits.next()?.trim().to_string();
    if splits.clone().count() == 3 {
        file += ":";
        file += splits.next()?.trim();
    }
    let row = splits.next()?.trim();
    let col = splits.next()?.trim();

    let location = types::Location {
        path: file.to_string(),
        row: row.parse().unwrap_or_default(),
        col: col.parse().unwrap_or_default(),
    };

    // remove decoration
    let desc = &desc[5..desc.len()];

    Some(types::Message {
        error: desc.to_string(),
        locations: vec![location],
    })
}

#[cfg(test)]
mod tests {
    use crate::{analyser::biome::analyse, core::types};
    use pretty_assertions::assert_eq;

    #[test]
    fn error_1() {
        static LOG: &str = include_str!("../../tests/biome_1.log");
        let result = analyse(LOG, "/tmp/project");

        assert_eq!(
            result,
            vec![types::Message {
                error: "Decorators are not valid here.".to_string(),
                locations: vec![types::Location {
                    path: "/tmp/project/src/app/util/functions/date-format.functions.ts"
                        .to_string(),
                    row: 44,
                    col: 15
                }]
            },]
        );
    }
}
