#include "dummy-call.h"

#include <call-ui.h>
#include <glib.h>
#include <glib/gi18n.h>
#include <gtk/gtk.h>

static gboolean
has_signal_handler_connected (CuiCallDisplay *display,
                              CuiCall        *call)
{
  gulong handler_id = g_signal_handler_find (call,
                                             G_SIGNAL_MATCH_DATA,
                                             0,
                                             0,
                                             NULL,
                                             NULL,
                                             display);
  return handler_id != 0;
}

static void
test_display (void)
{
  CuiCallDisplay *display;
  CuiDummyCall *call_one = cui_dummy_call_new ();
  CuiDummyCall *call_two = cui_dummy_call_new ();

  g_test_expect_message ("Cui", G_LOG_LEVEL_WARNING, "libcallaudio not initialized");

  display = cui_call_display_new (NULL);

  g_assert_true (CUI_IS_CALL_DISPLAY (display));
  g_assert_true (CUI_IS_CALL (call_one));
  g_assert_true (CUI_IS_CALL (call_two));

  g_assert_null (cui_call_display_get_call (display));

  cui_call_display_set_call (display, CUI_CALL (call_one));
  g_assert_true (CUI_CALL (call_one) == cui_call_display_get_call (display));

  g_assert_true (has_signal_handler_connected (display, CUI_CALL (call_one)));

  cui_call_display_set_call (display, CUI_CALL (call_two));
  g_assert_false (CUI_CALL (call_one) == cui_call_display_get_call (display));
  g_assert_true (CUI_CALL (call_two) == cui_call_display_get_call (display));

  g_assert_false (has_signal_handler_connected (display, CUI_CALL (call_one)));
  g_assert_true (has_signal_handler_connected (display, CUI_CALL (call_two)));

  cui_call_display_set_call (display, NULL);
  g_assert_false (CUI_CALL (call_one) == cui_call_display_get_call (display));
  g_assert_false (CUI_CALL (call_two) == cui_call_display_get_call (display));
  g_assert_null (cui_call_display_get_call (display));

  g_assert_false (has_signal_handler_connected (display, CUI_CALL (call_one)));
  g_assert_false (has_signal_handler_connected (display, CUI_CALL (call_two)));


  /* Now test if the display behaves when the set call is destroyed */
  cui_call_display_set_call (display, CUI_CALL (call_one));
  g_assert_true (CUI_CALL (call_one) == cui_call_display_get_call (display));

  g_assert_true (has_signal_handler_connected (display, CUI_CALL (call_one)));

  g_assert_finalize_object (call_one);

  g_assert_null (cui_call_display_get_call (display));

  /* Clean up */
  g_assert_finalize_object (call_two);
  gtk_widget_destroy (GTK_WIDGET (display));
}

int
main (int   argc,
      char *argv[])
{
  int retval;

  gtk_test_init (&argc, &argv, NULL);

  cui_init (FALSE);

  g_test_add_func ("/CallUI/display", test_display);

  retval = g_test_run ();

  cui_uninit ();

  return retval;
}
