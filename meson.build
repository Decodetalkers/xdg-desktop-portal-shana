project(
  'xdg-desktop-portal-shana',
  'rust',
  version: '0.0.1',
  meson_version : '>= 0.60'
)

cargo = find_program('cargo', version: '>= 1.63')

rustc = find_program('rustc', version: '>= 1.63')

if get_option('debug')
  command = [
    cargo, 'build',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'debug' / meson.project_name(), '@OUTPUT@'
  ]
else
  command = [
    cargo, 'build',
    '--release',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'release' / meson.project_name(), '@OUTPUT@'
  ]
endif

prefix = get_option('prefix')

xdg_install_dir = prefix / get_option('libexecdir')

portal_dir = prefix / get_option('datadir') / 'xdg-desktop-portal' / 'portals'

dbus1_dir = prefix / get_option('datadir') / 'dbus-1' / 'services'

xdg_target = custom_target('xdg-deskop-portal-shana',
  output: 'xdg-deskop-portal-shana',
  build_by_default: true,
  install: true,
  install_dir: xdg_install_dir,
  console: true,
  command: command
)

install_data('misc/shana.portal',
  install_dir: portal_dir
)

install_data('misc/org.freedesktop.impl.portal.desktop.shana.service',
  install_dir: dbus1_dir
)
