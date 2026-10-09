desktop_conf = configuration_data()
desktop_conf.set('app-id', app_id)

desktop_file = configure_file(
  input: 'dev.keinsell.Counter.desktop.in',
  output: 'dev.keinsell.Counter.desktop',
  configuration: desktop_conf,
)

install_data(
  desktop_file,
  install_dir: join_paths(datadir, 'applications'),
)

desktop_file_validate = find_program('desktop-file-validate', required: false)
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [desktop_file],
  )
endif

metainfo_conf = configuration_data()
metainfo_conf.set('app-id', app_id)

metainfo_file = configure_file(
  input: 'dev.keinsell.Counter.metainfo.xml.in',
  output: 'dev.keinsell.Counter.metainfo.xml',
  configuration: metainfo_conf,
)

install_data(
  metainfo_file,
  install_dir: join_paths(datadir, 'metainfo'),
)

appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test(
    'validate-metainfo',
    appstreamcli,
    args: ['validate', '--no-net', metainfo_file],
  )
endif

resource_files = files('counter.gresource.xml')

install_data(
  'icons/hicolor/scalable/apps/@0@.svg'.format(base_id),
  install_dir: join_paths(datadir, 'icons', 'hicolor', 'scalable', 'apps'),
)

install_data(
  'icons/hicolor/symbolic/apps/@0@-symbolic.svg'.format(base_id),
  install_dir: join_paths(datadir, 'icons', 'hicolor', 'symbolic', 'apps'),
)
