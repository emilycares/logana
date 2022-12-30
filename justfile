default:
    echo 'Hello, world!'
clippy:
  cargo clippy --fix -- \
  -W clippy::padantic \
  -W clippy::nursery \
  -W clippy::unwrap_used \
