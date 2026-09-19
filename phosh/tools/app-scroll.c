/*
 * Copyright © 2019 Zander Brown <zbrown@gnome.org>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Zander Brown <zbrown@gnome.org>
 */

#include <adwaita.h>
#include <activity.h>


static void
css_setup (void)
{
  GtkCssProvider *provider = NULL;
  GFile *file = NULL;

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
  GtkScrolledWindow *scrolled;
  GtkBox *box;
  GtkWidget *activity;

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "default-height", 640,
                         "default-width", 360,
                         "height-request", 640,
                         "width-request", 360,
                         "title", "PhoshApp Demo",
                         NULL);

  scrolled = g_object_new (GTK_TYPE_SCROLLED_WINDOW,
                           "vscrollbar-policy", GTK_POLICY_NEVER,
                           NULL);
  gtk_window_set_child (window, GTK_WIDGET (scrolled));

  box = g_object_new (GTK_TYPE_BOX,
                      "spacing", 18,
                      "margin-start", 24,
                      "margin-end", 24,
                      "margin-top", 10,
                      "margin-bottom", 10,
                      "halign", GTK_ALIGN_CENTER,
                      "valign", GTK_ALIGN_FILL,
                      NULL);
  gtk_scrolled_window_set_child (scrolled, GTK_WIDGET (box));

  activity = g_object_new (PHOSH_TYPE_ACTIVITY,
                      "app-id", "org.gnome.Calculator",
                      "title", "1 + 1 = 2",
                      "win-width", 360,
                      "win-height", 640,
                      NULL);
  gtk_box_append (box, activity);

  activity = g_object_new (PHOSH_TYPE_ACTIVITY,
                      "app-id", "org.gnome.Nautilus",
                      "title", "Home",
                      "win-width", 640,
                      "win-height", 360,
                      NULL);
  gtk_box_append (box, activity);

  gtk_window_present (window);
}


int
main (int argc, char **argv)
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.AppScrollDemo",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
