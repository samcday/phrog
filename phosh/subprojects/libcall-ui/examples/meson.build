if get_option('examples')

call_ui_demo_resources = gnome.compile_resources(
   'call-ui-demo-resources',
   'call-ui-demo.gresources.xml',

   c_name: 'cui_demo',
)

call_ui_demo_sources = [
  call_ui_demo_resources,
  'call-ui-demo.c',
  'cui-demo-call.c',
  'cui-demo-window.c',
]

call_ui_demo = executable('call-ui-demo',
  call_ui_demo_sources,
  dependencies: libcall_ui_dep,
  gui_app: true,
  include_directories: [ root_inc ],
  install: false,
)

endif
