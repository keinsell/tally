project(
  'counter',
  'rust',
  license: 'MIT',
  meson_version: '>= 1.4',
  version: '0.1.0'
)

gnome = import('gnome')

prefix      = get_option('prefix')
datadir     = get_option('datadir')
bindir      = get_option('bindir')
profile     = get_option('profile')

base_id = 'dev.keinsell.Counter'
app_id = base_id
if profile == 'Devel'
  app_id += '.Devel'
endif

glib_version = '2.66.0'
gio = dependency('gio-2.0', version: '>= @0@'.format(glib_version))
gtk = dependency('gtk4', version: '>= 4.0')
adw = dependency('libadwaita-1', version: '>= 1.1')
blueprintc = find_program('blueprint-compiler', version: '>= 0.22')
cargo = find_program('cargo')

subdir('data')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
