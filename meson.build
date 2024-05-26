project('gmobile', 'c',
        version: '0.2.0',
        license: 'LGPLv2.1+',
        meson_version: '>= 0.56.1',
        default_options: [ 'warning_level=1', 'buildtype=debugoptimized', 'c_std=gnu11' ],
)

cc = meson.get_compiler('c')

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
localedir = prefix / get_option('localedir')
libdir = prefix / get_option('libdir')
pkgdatadir = datadir / meson.project_name()
pkglibdir = libdir / meson.project_name()
udevdir = prefix / 'lib' / 'udev'

global_c_args = ['-DGMOBILE_COMPILATION']

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
  '-Wno-missing-field-initializers',
  '-Wno-sign-compare',
  '-Wno-strict-aliasing',
  '-Wno-unused-parameter',
  '-Wold-style-definition',
  '-Wpointer-arith',
  '-Wredundant-decls',
  '-Wshadow',
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

epoll_dep = dependency('epoll-shim', required: false)
glib_dep = dependency('glib-2.0', version: '>=2.66')
gio_dep = dependency('gio-2.0', version: '>=2.66')
json_glib_dep = dependency('json-glib-1.0', version: '>= 1.6.2',
                   fallback: ['json-glib', 'json_glib_dep'])

foreach arg: test_c_args
  if cc.has_multi_arguments(arg)
    global_c_args += arg
  endif
endforeach

gnome = import('gnome')

add_project_arguments(
  global_c_args,
  language: 'c'
)

config_h = configuration_data()
config_h.set_quoted('GM_VERSION', meson.project_version())

root_inc = include_directories('.')
gm_config_h = configure_file(
  output: 'gm-config.h',
  configuration: config_h,
)

subdir('data')
subdir('src')
subdir('tests')
subdir('examples')
subdir('doc')

summary({
    'Examples': get_option('examples'),
    'Tests': get_option('tests'),
    'Introspection': get_option('introspection'),
    'Docmentation': get_option('gtk_doc'),
    'Manual pages': get_option('man'),
    'Hwdb': get_option('hwdb'),
  },
  bool_yn: true,
  section: 'Build',
)
