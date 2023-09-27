set positional-arguments

watch +args='test':
  cargo watch --clear --exec '{{args}}'

ci: clippy forbid
  cargo fmt -- --check
  cargo test --all
  cargo test --all -- --ignored

forbid:
  ./bin/forbid

clippy:
  cargo clippy --all --all-targets -- -D warnings
