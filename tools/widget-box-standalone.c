/*
 * Copyright (C) 2022 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 *
 * BUILDDIR $ ./tools/run_tool ./tools/widget-box
 *
 * widget-box is a simple wrapper to run phosh's lockscreen widgets
 */

#include "phosh-config.h"

#include <adwaita.h>

#include <widget-box.h>
#include <plugin-loader.h>


static GStrv
get_plugin_dirs (GStrv plugins)
{
  g_autoptr (GPtrArray) dirs = g_ptr_array_new_with_free_func (g_free);

  for (int i = 0; i < g_strv_length (plugins); i++) {
    char *dir = g_strdup_printf (BUILD_DIR "/plugins/%s", plugins[i]);
    g_ptr_array_add (dirs, dir);
  }
  g_ptr_array_add (dirs, NULL);

  return (GStrv) g_ptr_array_steal (dirs, NULL);
}


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
  g_auto (GStrv) plugins = g_strsplit (PLUGINS, " ", -1);
  g_auto (GStrv) plugin_dirs = NULL;
  PhoshWidgetBox *box;

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "title", "Lockscreen Widget Box",
                         "default-width", 360,
                         "default-height", 720,
                         NULL);

  plugin_dirs = get_plugin_dirs (plugins);
  box = g_object_new (PHOSH_TYPE_WIDGET_BOX, "plugin-dirs", plugin_dirs, NULL);
  phosh_widget_box_set_plugins (box, plugins);
  gtk_window_set_child (window, GTK_WIDGET (box));

  gtk_window_present (window);
}


int
main (int argc, char *argv[])
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.WidgetBoxStandalone",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
