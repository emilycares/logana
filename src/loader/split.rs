pub fn split_builds<'a>(log: &'a str, split_symbol: &str) -> Vec<String> {
    let mut out = vec![];
    let lines: Vec<&str> = log.split('\n').collect();
    let last = lines.len();

    let split_lines: Vec<usize> = lines
        .to_owned()
        .into_iter()
        .enumerate()
        .filter(|(_, line)| line.trim().starts_with(&split_symbol))
        .map(|(i, _)| i)
        .collect();

    for n in 0..last {
        if let Some(split_line) = split_lines.get(n) {
            if n != 0 {
                if let Some(start) = split_lines.get(n - 1) {
                    out.push(combine_lines(*start, *split_line, &lines));
                } else {
                    out.push(combine_lines(0, *split_line, &lines));
                }
            }

            if n == split_lines.len() - 1 {
                out.push(combine_lines(*split_line, last, &lines));
            }
        }
    }

    out
}

fn combine_lines(start: usize, end: usize, lines: &Vec<&str>) -> String {
    let mut out = String::new();
    for n in start..end {
        if let Some(line) = lines.get(n) {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::loader::split::split_builds;

    #[test]
    fn should_split_builds() {
        static LOG: &'static str = include_str!("../../tests/cargo_split_1.log");

        let out = split_builds(LOG, "michael@dione ");

        assert_eq!(out, vec![
                   "michael@dione ~/t/some_project (main)> cargo build\n   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)\n    Finished dev [unoptimized + debuginfo] target(s) in 0.43s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n    Finished dev [unoptimized + debuginfo] target(s) in 0.00s\n",
                   "michael@dione ~/t/some_project (main)> nvim src/main.rs\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)\nerror[E0425]: cannot find value `asd` in this scope\n --> src/main.rs:2:5\n  |\n2 |     asd\n  |     ^^^ not found in this scope\n\nFor more information about this error, try `rustc --explain E0425`.\nerror: could not compile `some_project` due to previous error\n",
                   "michael@dione ~/t/some_project (main) [101]> cargo build\n   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)\nerror[E0425]: cannot find value `asd` in this scope\n --> src/main.rs:2:5\n  |\n2 |     asd\n  |     ^^^ not found in this scope\n\nFor more information about this error, try `rustc --explain E0425`.\nerror: could not compile `some_project` due to previous error\n",
                   "michael@dione ~/t/some_project (main) [101]> nvim src/main.rs\n",
                   "michael@dione ~/t/some_project (main)> cargo build\n   Compiling some_project v0.1.0 (/home/michael/tmp/some_project)\n    Finished dev [unoptimized + debuginfo] target(s) in 0.14s\n\n"])
    }
}
