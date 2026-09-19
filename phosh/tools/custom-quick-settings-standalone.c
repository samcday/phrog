/*
 * Copyright (C) 2024 Tether Operations Limited
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Arun Mani J <arunmani@peartree.to>
 */

#define G_LOG_DOMAIN "phosh-custom-quick-settings"

/**
 * A widget to test custom quick settings
 *
 * BUILDIR $ ./tools/run-tool ./tools/custom-quick-settings
 */

#include "phosh-config.h"

#include <adwaita.h>

#include "plugin-loader.h"
#include "quick-setting.h"
#include "quick-settings-box.h"


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


static GtkWidget *
setup_plugins (const char *const *enabled)
{
  PhoshQuickSettingsBox *box;
  g_auto (GStrv) plugins = g_strsplit (PLUGINS, " ", -1);
  g_auto (GStrv) plugin_dirs = NULL;
  g_autoptr (PhoshPluginLoader) loader = NULL;

  plugin_dirs = get_plugin_dirs (plugins);

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 12));
  loader = phosh_plugin_loader_new (plugin_dirs, PHOSH_EXTENSION_POINT_QUICK_SETTING_WIDGET);

  for (int i = 0; i < g_strv_length (plugins); i++) {
    char *plugin = plugins[i];
    PhoshQuickSetting* widget;

    if (!g_strv_contains (enabled, plugin))
      continue;

    widget = PHOSH_QUICK_SETTING (phosh_plugin_loader_load_plugin (loader, plugin));
    if (widget == NULL) {
      g_warning ("Unable to load plugin: %s", plugin);
    } else {
      g_print ("Adding custom quick setting '%s'\n", plugin);
      phosh_quick_settings_box_add (box, widget);
    }
  }

  return GTK_WIDGET (box);
}


static void
on_activate (AdwApplication *app, const char *const *enabled)
{
  GtkWidget *box;
  GtkWindow *window;

  css_setup ();

  box = setup_plugins (enabled);
  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "title", "Custom Quick Settings",
                         NULL);
  gtk_window_set_child (window, box);

  gtk_window_present (window);
}


int
main (int argc, char *argv[])
{
  g_autoptr (GOptionContext) opt_context = NULL;
  g_autoptr (GError) err = NULL;
  g_autoptr (GStrvBuilder) plugins_builder = g_strv_builder_new ();
  g_auto (GStrv) enabled = NULL;
  const GOptionEntry options [] = {
    { NULL, 0, 0, G_OPTION_ARG_NONE, NULL, NULL, NULL }
  };
  g_autoptr (AdwApplication) app = NULL;

  opt_context = g_option_context_new ("- spawn your quick setting");
  g_option_context_add_main_entries (opt_context, options, NULL);
  if (!g_option_context_parse (opt_context, &argc, &argv, &err)) {
    g_warning ("%s", err->message);
    return 1;
  }

  if (argc < 2) {
    g_print ("Pass at least one plugin name\n");
    return 1;
  }

  for (int i = 1; i < argc; i++)
    g_strv_builder_add (plugins_builder, argv[i]);
  enabled = g_strv_builder_end (plugins_builder);

  app = adw_application_new ("mobi.phosh.tools.CustomQuickSettingsStandalone",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), enabled);
  return g_application_run (G_APPLICATION (app), 0, NULL);
  return 0;
}
