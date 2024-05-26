if get_option('tests')

test_env = [
  'G_DEBUG=gc-friendly,fatal-warnings',
  'MALLOC_CHECK_=2',
  'NO_AT_BRIDGE=1',
]

test_cflags = [
  '-DTEST_DATA_DIR="@0@"'.format(meson.current_source_dir() / 'data'),
]

tests = [
  'device-tree',
  'display-panel',
  'cutout',
  'svg-path',
  'timeout',
  'utils',
]

foreach test : tests
  t = executable('test-@0@'.format(test),
                 ['test-@0@.c'.format(test)],
		 c_args : test_cflags,
                 pie: true,
                 link_with: gm_lib,
                 dependencies: gmobile_dep)
  test(test, t, env: test_env)
endforeach

endif
