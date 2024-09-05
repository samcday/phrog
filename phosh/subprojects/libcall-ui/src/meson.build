call_ui_enum_headers = files(['cui-call.h',
                             ])
call_ui_enum_sources = gnome.mkenums_simple('cui-enums',
                                            sources : call_ui_enum_headers)

call_ui_resources = gnome.compile_resources(
   'cui-resources',
   'call-ui.gresources.xml',
   extra_args: '--manual-register',
   c_name: 'cui',
)

call_ui_deps = [
  libcallaudio_dep,
  gio_dep,
  gtk_dep,
  libhandy_dep,
  cc.find_library('m', required: false),
  cc.find_library('rt', required: false),
]

call_ui_private_sources = files(
  'cui-encryption-indicator.c',
  'cui-encryption-indicator-priv.h',
  'cui-keypad-button.c',
  'cui-keypad-button-private.h'
)

call_ui_public_sources = files(
 'cui-call.c',
 'cui-call.h',
 'cui-call-display.c',
 'cui-call-display.h',
 'cui-main.c',
 'cui-main.h',
 'cui-dialpad.c',
 'cui-dialpad.h',
 'cui-keypad.c',
 'cui-keypad.h',
)

call_ui_sources = [
  call_ui_public_sources,
  call_ui_private_sources,
  call_ui_resources,
  call_ui_enum_sources,
]

call_ui_c_args = [
  '-DG_LOG_DOMAIN="Cui"',
]

call_ui_lib = static_library('call-ui',
  call_ui_sources,
  c_args: call_ui_c_args,
  include_directories: [ root_inc, src_inc ],
  dependencies: call_ui_deps)

libcall_ui_dep = declare_dependency(
  sources: call_ui_enum_sources,
  include_directories: include_directories('.'),
  dependencies: call_ui_deps,
  link_with: call_ui_lib)

if get_option('gtk_doc')
  libcall_ui_gir_extra_args = [
    '--c-include=call-ui.h',
    '-L@0@'.format(meson.current_build_dir()),
    '--quiet',
    '-DLIBCALL_UI_COMPILATION',
   ]

  libcall_ui_gir = gnome.generate_gir(call_ui_lib,
                 sources: call_ui_sources,
               nsversion: '0',
               namespace: 'Cui',
         export_packages: 'call-ui-0',
           symbol_prefix: 'cui',
       identifier_prefix: 'Cui',
               link_with: call_ui_lib,
                includes: ['Gio-2.0', 'Gtk-3.0', 'Handy-1'],
                 install: true,
              extra_args: libcall_ui_gir_extra_args,
   )

endif
