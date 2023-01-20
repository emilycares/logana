clippy:
  logana -c "cargo clippy --color always --fix --allow-staged --allow-dirty -- \
  -W clippy::pedantic \
  -W clippy::nursery \
  -W clippy::unwrap_used"
build:
  logana -c "cargo build --color always"
test:
  logana -c "cargo test --color always"
test-spec:
  logana -c "cargo test input::tests::split_analyse_should_work_as_intended --color always -- --nocapture"
