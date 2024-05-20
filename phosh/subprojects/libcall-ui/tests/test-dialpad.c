#include <call-ui.h>
#include <glib.h>
#include <glib/gi18n.h>
#include <gtk/gtk.h>

static void
test_dialpad (void)
{
  CuiDialpad *dialpad;
  GValue val = G_VALUE_INIT;

  g_test_expect_message ("Cui", G_LOG_LEVEL_WARNING, "libcallaudio not initialized");

  dialpad = cui_dialpad_new ();

  g_assert_true (CUI_IS_DIALPAD (dialpad));

  /* test number proprietary */
  g_value_init (&val, G_TYPE_STRING);
  g_value_set_string (&val, "+3129877983#*3129877983");

  g_object_set_property (G_OBJECT (dialpad), "number", &val);

  g_value_set_string (&val, "Don't be this string!");

  g_object_get_property (G_OBJECT (dialpad), "number", &val);

  g_assert_cmpstr (g_value_get_string (&val), ==, "+3129877983#*3129877983");

  /* Clean up */
  g_value_unset (&val);
  gtk_widget_destroy (GTK_WIDGET (dialpad));
}

int
main (int   argc,
      char *argv[])
{
  int retval;

  gtk_test_init (&argc, &argv, NULL);

  cui_init (FALSE);

  g_test_add_func ("/CallUI/dialpad", test_dialpad);

  retval = g_test_run ();

  cui_uninit ();

  return retval;
}
