gm_resources = gnome.compile_resources(
  'gm-resources',
  'gmobile.gresources.xml',
  extra_args: '--manual-register',
  c_name: 'gm',
)

if get_option('hwdb')
  install_data('61-gmobile-wakeup.hwdb',
    install_dir: udevdir / 'hwdb.d',
   )
  install_data('61-gmobile.rules',
    install_dir: udevdir / 'rules.d',
   )
endif
