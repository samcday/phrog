gm_deps = [
  epoll_dep,
  gio_dep,
  glib_dep,
  json_glib_dep,
  cc.find_library('m', required: false),
  cc.find_library('rt', required: false),
]

gm_public_sources = files(
  'gm-cutout.c',
  'gm-device-info.c',
  'gm-device-tree.c',
  'gm-display-panel.c',
  'gm-error.c',
  'gm-main.c',
  'gm-rect.c',
  'gm-svg-path.c',
  'gm-timeout.c',
  'gm-util.c',
)

gm_public_headers = files(
  'gmobile.h',
  'gm-cutout.h',
  'gm-device-info.h',
  'gm-device-tree.h',
  'gm-display-panel.h',
  'gm-error.h',
  'gm-main.h',
  'gm-rect.h',
  'gm-svg-path.h',
  'gm-timeout.h',
  'gm-util.h',
)
install_headers(gm_public_headers + [gm_config_h], subdir: 'gmobile')

gm_sources = [
  gm_public_sources,
  gm_public_headers,
  gm_resources,
]

gm_c_args = [
  '-DG_LOG_DOMAIN="gmobile"',
]

gm_lib = both_libraries(
  'gmobile',
  gm_sources,
  include_directories: root_inc,
  c_args: gm_c_args,
  dependencies: gm_deps,
  soversion: '0',
  install: true)

gmobile_dep = declare_dependency(
  include_directories: [root_inc, include_directories('.')],
  dependencies: gm_deps,
  link_with: gm_lib.get_static_lib())

gmobile_shared_dep = declare_dependency(
  include_directories: [root_inc, include_directories('.')],
  dependencies: gm_deps,
  link_with: gm_lib.get_shared_lib())

pkgconfig = import('pkgconfig')
pkgconfig.generate(
  gm_lib,
  subdirs: 'gmobile',
  description: 'Functions useful in mobile related, glib based projects',
  url: 'https://world.pages.gitlab.gnome.org/Phosh/gmobile/',
)

if get_option('gtk_doc') or get_option('introspection')
  gmobile_gir_extra_args = [
    '--c-include=gmobile.h',
    '-L@0@'.format(meson.current_build_dir()),
    '--quiet',
    '-DGMOBILE_USE_UNSTABLE_API',
   ]

  gmobile_gir = gnome.generate_gir(gm_lib,
                 sources: gm_public_sources + gm_public_headers,
               nsversion: '0',
               namespace: 'Gm',
         export_packages: 'gmobile',
           symbol_prefix: 'gm',
       identifier_prefix: 'Gm',
                includes: ['Gio-2.0'],
                 install: get_option('introspection'),
              extra_args: gmobile_gir_extra_args,
   )
endif
