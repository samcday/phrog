/*
 * Copyright © 2019 Zander Brown <zbrown@gnome.org>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Zander Brown <zbrown@gnome.org>
 */

#include "gio/gio.h"
#include <adwaita.h>
#include <app-grid-button.h>


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
  GtkBox *wrap;
  GtkBox *box;
  GtkButton *btn;
  GDesktopAppInfo *info;

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "default-height", 100,
                         "default-width", 360,
                         "height-request", 100,
                         "width-request", 360,
                         "title", "PhoshAppGridButton Demo",
                         NULL);
  wrap = g_object_new (GTK_TYPE_BOX,
                      "spacing", 20,
                      "orientation", GTK_ORIENTATION_VERTICAL,
                      "margin-start", 6,
                      "margin-end", 6,
                      "margin-top", 6,
                      "margin-bottom", 6,
                      "halign", GTK_ALIGN_CENTER,
                      "valign", GTK_ALIGN_CENTER,
                      NULL);
  gtk_window_set_child (window, GTK_WIDGET (wrap));
  box = g_object_new (GTK_TYPE_BOX,
                      "spacing", 20,
                      "margin-start", 6,
                      "margin-end", 6,
                      "margin-top", 6,
                      "margin-bottom", 6,
                      "halign", GTK_ALIGN_CENTER,
                      "valign", GTK_ALIGN_CENTER,
                      NULL);
  gtk_box_append (wrap, GTK_WIDGET (box));

  info = g_desktop_app_info_new ("org.gtk.Demo4.desktop");
  btn = g_object_new (PHOSH_TYPE_APP_GRID_BUTTON,
                      "app-info", info,
                      "mode", PHOSH_APP_GRID_BUTTON_FAVORITES,
                      "visible", TRUE,
                      NULL);
  gtk_box_append (box, GTK_WIDGET (btn));

  info = g_desktop_app_info_new ("org.gtk.IconBrowser4.desktop");
  btn = g_object_new (PHOSH_TYPE_APP_GRID_BUTTON,
                      "app-info", info,
                      "mode", PHOSH_APP_GRID_BUTTON_FAVORITES,
                      "visible", TRUE,
                      NULL);
  gtk_box_append (box, GTK_WIDGET (btn));

  box = g_object_new (GTK_TYPE_BOX,
                      "spacing", 20,
                      "margin-start", 6,
                      "margin-end", 6,
                      "margin-top", 6,
                      "margin-bottom", 6,
                      "halign", GTK_ALIGN_CENTER,
                      "valign", GTK_ALIGN_CENTER,
                      NULL);
  gtk_box_append (wrap, GTK_WIDGET (box));

  info = g_desktop_app_info_new ("org.gtk.Demo4.desktop");
  btn = g_object_new (PHOSH_TYPE_APP_GRID_BUTTON,
                      "app-info", info,
                      "visible", TRUE,
                      NULL);
  gtk_box_append (box, GTK_WIDGET (btn));

  info = g_desktop_app_info_new ("org.gtk.IconBrowser4.desktop");
  btn = g_object_new (PHOSH_TYPE_APP_GRID_BUTTON,
                      "app-info", info,
                      "visible", TRUE,
                      NULL);
  gtk_box_append (box, GTK_WIDGET (btn));

  gtk_window_present (window);
}


int
main (int argc, char **argv)
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.AppGridButtonDemo",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
