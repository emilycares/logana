/// A utility function to split multipel builds
#[must_use]
pub fn builds<'a>(lines: &'a [&'a str], split_symbol: &'a str) -> Vec<&'a [&'a str]> {
    let mut out = vec![];
    let last = lines.len();

    let split_lines: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(id, line)| {
            if line.trim().contains(split_symbol) {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    for n in 0..last {
        if let Some(split_line) = split_lines.get(n) {
            if n != 0 {
                if let Some(start) = split_lines.get(n - 1) {
                    out.push(combine_lines(*start, *split_line, lines));
                } else {
                    out.push(combine_lines(0, *split_line, lines));
                }
            }

            if n == split_lines.len() - 1 {
                out.push(combine_lines(*split_line, last, lines));
            }
        }
    }

    out
}

fn combine_lines<'a>(start: usize, end: usize, lines: &'a [&'a str]) -> &[&'a str] {
    &lines[start..end]
}

#[cfg(test)]
mod tests {
    use crate::loader::split::builds;
    use pretty_assertions::assert_eq;

    //#[test]
    //fn should_split_builds() {
    //static LOG: &str = include_str!("../../tests/cargo_split_1.log");
    //let lines: Vec<&str> = LOG.lines().collect();
    //let lines = lines.as_slice();

    //let out = builds(lines, "michael@dione ");

    //assert_eq!(out, [
    //["michael@dione ~/t/some_project (main)> cargo build","   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)", "    Finished dev [unoptimized + debuginfo] target(s) in 0.43s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "   Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> cargo build", "    Finished dev [unoptimized + debuginfo] target(s) in 0.00s"],
    //["michael@dione ~/t/some_project (main)> nvim src/main.rs"],
    //["michael@dione ~/t/some_project (main)> cargo build", "   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)", "error[E0425]: cannot find value `asd` in this scope", " --> src/main.rs:2:5", "  |", " |     asd", "  |     ^^^ not found in this scope", "For more information about this error, try `rustc --explain E0425`.", ": could not compile `some_project` due to previous error"],
    //["michael@dione ~/t/some_project (main) [101]> cargo build", "   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)", "[E0425]: cannot find value `asd` in this scope", " --> src/main.rs:2:5", "  |", "2 |     asd", "  |     ^^^ not found in this scope", "For more information about this error, try `rustc --explain E0425`.", "error: could not compile `some_project` due to previous error"],
    //["michael@dione ~/t/some_project (main) [101]> nvim src/main.rs"],
    //"michael@dione ~/t/some_project (main)> cargo build", "   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)", "    Finished dev [unoptimized + debuginfo] target(s) in 0.14s"]
    //]);
    //}

    //#[test]
    //fn should_split_builds_2() {
    //static LOG: &str = include_str!("../../tests/cargo_split_2.log");
    //let lines: Vec<&str> = LOG.lines().collect();
    //let lines = lines.as_slice();

    //let out = builds(lines, "michael@dione ");

    //assert_eq!(out, vec![
    //["michael@dione ~/D/r/moxy (master)> cargo build                ", "   Compiling moxy v0.1.0 (/home/michael/Documents/rust/moxy)", ": expected `;`, found `#`", " --> src/main.rs:2:16", "  |", " | pub mod builder", "  |                ^ help: add `;` here", " | #[warn(missing_docs)]", "  | - unexpected token", ",: expected item, found `<eof>`", "  --> src/main.rs:22:1", "   |", " | }", "   | ^ expected item", "error: could not compile `moxy` due to 2 previous errors"],
    //["michael@dione ~/D/r/moxy (master) [101]>"],
    //]);
    //}

    #[test]
    fn should_split_builds_3() {
        static LOG: &str = include_str!("../../tests/split_analyse_1.log");
        let lines: Vec<&str> = LOG.lines().collect();
        let lines = lines.as_slice();

        let out = builds(lines, "Browser application bundle generation complete");

        assert_eq!(out, vec![
                   ["TypeError: 0 Cannot read property 'component' of undefined", "      at MapSubscriber.call [as project] (http://localhost:9876/_karma_webpack_/webpack:/src/app/components/layout/main/command-info-dialog-modal/command-info-dialog-modal.component.ts:83:1)"],
                   ["TypeError: 1 Cannot read property 'component' of undefined", "      at MapSubscriber.call [as project] (http://localhost:9876/_karma_webpack_/webpack:/src/app/components/layout/main/command-info-dialog-modal/command-info-dialog-modal.component.ts:83:1)"],
                   ["TypeError: 2 Cannot read property 'component' of undefined", "      at MapSubscriber.call [as project] (http://localhost:9876/_karma_webpack_/webpack:/src/app/components/layout/main/command-info-dialog-modal/command-info-dialog-modal.component.ts:83:1)"],
        ]);
    }
}
