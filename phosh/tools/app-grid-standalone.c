/*
 * Copyright © 2019 Zander Brown <zbrown@gnome.org>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * BUILDDIR $ ./run_tool ./tools/app-grid-standalone
 *
 * PhoshAppGrid in a simple wrapper
 */

#include <app-grid.h>
#include <adwaita.h>


static void
css_setup (void)
{
  GtkCssProvider *provider;
  GFile *file;

  provider = gtk_css_provider_new ();
  file = g_file_new_for_uri ("resource:///mobi/phosh/stylesheet/adwaita-dark.css");

  gtk_css_provider_load_from_file (provider, file);
  gtk_style_context_add_provider_for_display (gdk_display_get_default (),
                                              GTK_STYLE_PROVIDER (provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
  g_object_unref (file);

  g_object_set (adw_style_manager_get_default (),
                "color-scheme", ADW_COLOR_SCHEME_FORCE_DARK,
                NULL);
}


static void
on_activate (AdwApplication *app)
{
  GtkWindow *window;
  GtkWidget *grid;

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "title", "AppGridStandalone",
                         NULL);
  grid = g_object_new (PHOSH_TYPE_APP_GRID, NULL);
  gtk_window_set_child (window, grid);

  gtk_window_present (window);
}


int
main (int argc, char *argv[])
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.AppGridStandalone",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
