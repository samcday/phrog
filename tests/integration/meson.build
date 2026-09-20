run_data = configuration_data()
run_data.set('ABS_BUILDDIR', meson.project_build_root())
run_data.set('ABS_SRCDIR', meson.project_source_root())
configure_file(
  input: 'run-pytest.in',
  output: 'run-pytest',
  configuration: run_data,
)
