project(
  'xdg-desktop-portal-shana',
  'rust',
  version: '0.3.17',
  meson_version: '>= 1.1.0',
)

cargo = find_program('cargo', version: '>= 1.80')

find_program('rustc', version: '>= 1.80')

if get_option('debug')
  command = [
    cargo,
    'build',
    '&&',
    'cp',
    meson.global_source_root() / 'target' / 'debug' / meson.project_name(),
    '@OUTPUT@',
  ]
else
  command = [
    cargo,
    'build',
    '--release', '&&',
    'cp',
    meson.global_source_root() / 'target' / 'release' / meson.project_name(),
    '@OUTPUT@',
  ]
endif

prefix = get_option('prefix')

xdg_install_dir = prefix / get_option('libexecdir')

portal_dir = prefix / get_option('datadir') / 'xdg-desktop-portal' / 'portals'

dbus1_dir = prefix / get_option('datadir') / 'dbus-1' / 'services'

system_dir = prefix / get_option('libdir') / 'systemd' / 'user'
global_conf = configuration_data()

global_conf.set('xdg_install_dir', xdg_install_dir)

systemd_file = configure_file(
  input: 'misc/xdg-desktop-portal-shana.service.in',
  output: 'xdg-desktop-portal-shana.service',
  configuration: global_conf,
)

dbus_file = configure_file(
  input: 'misc/org.freedesktop.impl.portal.desktop.shana.service.in',
  output: 'org.freedesktop.impl.portal.desktop.shana.service',
  configuration: global_conf,
)

custom_target(
  'xdg-desktop-portal-shana',
  output: 'xdg-desktop-portal-shana',
  build_by_default: true,
  install: true,
  install_dir: xdg_install_dir,
  console: true,
  command: command,
)

install_data('misc/shana.portal', install_dir: portal_dir)

install_data(
  dbus_file,
  install_dir: dbus1_dir,
)

install_data(
  systemd_file,
  install_dir: system_dir,
)
