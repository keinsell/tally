subdir('ui')

build_root = meson.project_build_root()
source_root = meson.project_source_root()

resources = gnome.compile_resources(
  'counter',
  resource_files,
  dependencies: ui,
  source_dir: [
    join_paths(source_root, 'data'),
    join_paths(meson.current_build_dir(), 'ui'),
  ],
  gresource_bundle: true
)

cargo_options = ['--manifest-path', source_root / 'Cargo.toml']
cargo_options += ['--target-dir', build_root / 'src']

cargo_env = {
  'APP_ID': app_id,
  'VERSION': meson.project_version(),
  'COUNTER_RESOURCES_FILE': resources.full_path(),
}

meson.add_devenv(cargo_env)

run_target(
  'cargo-check',
  command: [cargo, 'check', cargo_options],
  env: cargo_env,
)

run_target(
  'cargo-clippy',
  command: [cargo, 'clippy', cargo_options],
  env: cargo_env,
)

# Build the binary with cargo
cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'cargo-build',
  console: true,
  depends: [resources],
  env: cargo_env,
  command: [
    cargo,
    'build',
    cargo_options,
  ],
)

custom_target(
  'cp-binary',
  build_by_default: true,
  build_always_stale: true,
  depends: cargo_build,
  install: true,
  install_dir: get_option('bindir'),
  output: 'counter',
  command: [
    'cp',
    'src' / 'debug' / 'counter',
    '@OUTPUT@',
  ],
)
