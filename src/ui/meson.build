ui = custom_target(
  'ui',
  input: files('counter.blp'),
  output: '.',
  command: [blueprintc, 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)
