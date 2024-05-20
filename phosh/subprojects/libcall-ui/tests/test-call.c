#include <call-ui.h>

static void
test_utils (void)
{
  char *duration;

  duration = cui_call_format_duration (0);
  g_assert_cmpstr (duration, ==, "00:00");
  g_free (duration);
  
  duration = cui_call_format_duration (600.03);
  g_assert_cmpstr (duration, ==, "10:00");
  g_free (duration);
    
  duration = cui_call_format_duration (3600.00);
  g_assert_cmpstr (duration, ==, "60:00");
  g_free (duration);
  
  duration = cui_call_format_duration (3601.00);
  g_assert_cmpstr (duration, ==, "1:00:01");
  g_free (duration);
}


int
main (int   argc,
      char *argv[])
{
  int retval;

  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/CallUI/utils", test_utils);

  retval = g_test_run ();

  cui_uninit ();

  return retval;
}
