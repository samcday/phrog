#include "cui-config.h"

#include <gtk/gtk.h>
#include <call-ui.h>

#include <glib/gi18n.h>

#include "cui-demo-window.h"

static void
startup (GtkApplication *app)
{
  GtkCssProvider *css_provider = gtk_css_provider_new ();

  hdy_init ();
  cui_init (FALSE);

  gtk_css_provider_load_from_resource (css_provider, "/org/gnome/CallUI/Demo/ui/style.css");
  gtk_style_context_add_provider_for_screen (gdk_screen_get_default (),
                                             GTK_STYLE_PROVIDER (css_provider),
                                             GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
  g_object_unref (css_provider);
}

static void
show_window (GtkApplication *app)
{
  CuiDemoWindow *window;

  window = cui_demo_window_new (app);

  gtk_window_present (GTK_WINDOW (window));
}

int
main (int    argc,
      char **argv)
{
  GtkApplication *app;
  int status;

  /* This is enough since libcall-ui performs the bindtextdomain */
  textdomain (GETTEXT_PACKAGE);

  app = gtk_application_new ("org.gnome.CallUI.Demo",
#if GLIB_CHECK_VERSION (2, 73, 3)
                             G_APPLICATION_DEFAULT_FLAGS);
#else
                             G_APPLICATION_FLAGS_NONE);
#endif
  g_signal_connect (app, "startup", G_CALLBACK (startup), NULL);
  g_signal_connect (app, "activate", G_CALLBACK (show_window), NULL);
  status = g_application_run (G_APPLICATION (app), argc, argv);
  g_object_unref (app);

  return status;
}
