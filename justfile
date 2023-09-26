set positional-arguments

watch +args='test':
  cargo watch --clear --exec '{{args}}'
