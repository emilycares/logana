use crate::{analyser::clang, core::types};

#[must_use]
pub fn analyse(log: &str, project_dir: &str) -> Vec<types::Message> {
    clang::analyse(log, project_dir)
}
#[cfg(test)]
mod tests {
    use crate::{
        analyser::gcc::analyse,
        core::types::{Location, Message},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn error() {
        static LOG: &str = include_str!("../../tests/gcc_1.log");
        let result = analyse(LOG, "/tmp/project");
        assert_eq!(
            result,
            vec![
                Message {
                    error: "warning: unused parameter ‘argc’ [-Wunused-parameter]".to_string(),
                    locations: vec![Location {
                        path: "/tmp/project/./main.c".to_string(),
                        row: 315,
                        col: 14,
                    },],
                },
                Message {
                    error: "warning: control reaches end of non-void function [-Wreturn-type]"
                        .to_string(),
                    locations: vec![Location {
                        path: "/tmp/project/./main.c".to_string(),
                        row: 312,
                        col: 1,
                    },],
                },
            ]
        );
    }
}
