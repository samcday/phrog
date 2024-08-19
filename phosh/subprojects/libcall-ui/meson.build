project('libcall-ui', 'c',
          version: '0.1.1',
          license: 'LGPLv2.1+',
    meson_version: '>= 0.50.0',
  default_options: [ 'warning_level=1', 'buildtype=debugoptimized', 'c_std=gnu11' ],
)

prefix = get_option('prefix')
bindir = join_paths(prefix, get_option('bindir'))
datadir = join_paths(prefix, get_option('datadir'))
localedir = join_paths(prefix, get_option('localedir'))
libdir = join_paths(prefix, get_option('libdir'))
desktopdir = join_paths(datadir, 'applications')
sessiondir = join_paths(datadir, 'gnome-session')
pkgdatadir = join_paths(datadir, meson.project_name())
pkglibdir = join_paths(libdir, meson.project_name())
systemddir = join_paths(prefix, 'lib/systemd')
systemduserdir = join_paths(systemddir, 'user')

root_inc = include_directories('.')

cc = meson.get_compiler('c')

global_c_args = []
test_c_args = [
  '-Wcast-align',
  '-Wdate-time',
  '-Wdeclaration-after-statement',
  ['-Werror=format-security', '-Werror=format=2'],
  '-Wendif-labels',
  '-Werror=incompatible-pointer-types',
  '-Werror=missing-declarations',
  '-Werror=overflow',
  '-Werror=return-type',
  '-Werror=shift-count-overflow',
  '-Werror=shift-overflow=2',
  '-Werror=implicit-fallthrough=3',
  '-Wfloat-equal',
  '-Wformat-nonliteral',
  '-Wformat-security',
  '-Winit-self',
  '-Wmaybe-uninitialized',
  '-Wmissing-field-initializers',
  '-Wmissing-include-dirs',
  '-Wmissing-noreturn',
  '-Wnested-externs',
  '-Wno-strict-aliasing',
  '-Wno-unused-parameter',
  '-Wold-style-definition',
  '-Wpointer-arith',
  '-Wredundant-decls',
  '-Wshadow',
  '-Wsign-compare',
  '-Wstrict-prototypes',
  '-Wswitch-default',
  '-Wswitch-enum',
  '-Wtype-limits',
  '-Wundef',
  '-Wunused-function',
]
if get_option('buildtype') != 'plain'
  test_c_args += '-fstack-protector-strong'
endif

foreach arg: test_c_args
  if cc.has_multi_arguments(arg)
    global_c_args += arg
  endif
endforeach
add_project_arguments(
  global_c_args,
  language: 'c'
)

gnome = import('gnome')
i18n  = import('i18n')

root_inc = include_directories('.')
src_inc = include_directories('src')
tests_inc = include_directories('tests')

gio_dep = dependency('gio-2.0', version: '>= 2.62')
gtk_dep = dependency('gtk+-3.0', version: '>=3.22')
libhandy_dep = dependency('libhandy-1', version: '>=1.2')
libcallaudio_dep = dependency('libcallaudio-0.1')

config_h = configuration_data()
config_h.set_quoted('GETTEXT_PACKAGE', 'call-ui')
config_h.set_quoted('LOCALEDIR', localedir)
config_h.set_quoted('TOP_SOURCE_DIR', meson.current_source_dir())

configure_file(
  input: 'cui-config.h.in',
  output: 'cui-config.h',
  configuration: config_h,
)

#subdir('data')
subdir('po')
subdir('src')
subdir('examples')
subdir('tests')
subdir('doc')

summary = [
  '',
  '-----------',
  'libcall-ui @0@'.format(meson.project_version()),
  '',
  '        Tests: @0@'.format(get_option('tests')),
  'Documentation: @0@'.format(get_option('gtk_doc')),
  ' Install i18n: @0@'.format(get_option('install-i18n')),
  '-----------',
]

message('\n'.join(summary))
