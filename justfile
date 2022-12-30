clippy:
  cargo clippy --color always --fix --allow-staged --allow-dirty -- \
  -W clippy::pedantic \
  -W clippy::nursery \
  -W clippy::unwrap_used
