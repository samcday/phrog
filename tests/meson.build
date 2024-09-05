if get_option('tests')

test_env = [
  'G_DEBUG=gc-friendly,fatal-warnings',
  'MALLOC_CHECK_=2',
  'NO_AT_BRIDGE=1',
  'LSAN_OPTIONS=suppressions=@0@/data/leak-suppress.txt'.format(meson.source_root()),
  'ASAN_OPTIONS=fast_unwind_on_malloc=0',
]

display_test_sources = [
  'test-display.c',
  'dummy-call.c', 'dummy-call.h'
]

dialpad_test_sources = [
  'test-dialpad.c'
]

keypad_test_sources = [
  'test-keypad.c'
]

display_test = executable('display', display_test_sources,
               call_ui_resources,
               pie: true,
               link_with : call_ui_lib,
               dependencies : libcall_ui_dep,
               include_directories : [src_inc, tests_inc]
)
dialpad_test = executable('dialpad', dialpad_test_sources,
               call_ui_resources,
               pie: true,
               link_with : call_ui_lib,
               dependencies : libcall_ui_dep,
               include_directories : [src_inc, tests_inc]
)
keypad_test = executable('keypad', keypad_test_sources,
               call_ui_resources,
               pie: true,
               link_with : call_ui_lib,
               dependencies : libcall_ui_dep,
               include_directories : [src_inc, tests_inc]
)
call_test = executable('call', 'test-call.c',
               call_ui_resources,
               pie: true,
               link_with : call_ui_lib,
               dependencies : libcall_ui_dep,
               include_directories : [src_inc, tests_inc]
)

test ('display', display_test, env: test_env)

test ('dialpad', dialpad_test, env: test_env)

test ('keypad', keypad_test, env: test_env)

test ('call', call_test, env: test_env)

endif
