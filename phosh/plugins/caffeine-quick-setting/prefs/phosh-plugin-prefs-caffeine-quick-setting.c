/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Rudra Pratap Singh <rudrasingh900@gmail.com>
 */

#include "phosh-plugin-prefs-config.h"

#include "caffeine-quick-setting-prefs.h"

#include "phosh-plugin.h"

char **g_io_phosh_plugin_prefs_caffeine_quick_setting_query (void);

void
g_io_module_load (GIOModule *module)
{
  g_type_module_use (G_TYPE_MODULE (module));

  g_io_extension_point_implement (PHOSH_PLUGIN_EXTENSION_POINT_QUICK_SETTING_WIDGET_PREFS,
                                  PHOSH_TYPE_CAFFEINE_QUICK_SETTING_PREFS,
                                  PLUGIN_PREFS_NAME,
                                  10);
}


void
g_io_module_unload (GIOModule *module)
{
}


char **
g_io_phosh_plugin_prefs_caffeine_quick_setting_query (void)
{
  char *extension_points[] = { PHOSH_PLUGIN_EXTENSION_POINT_QUICK_SETTING_WIDGET_PREFS, NULL };

  return g_strdupv (extension_points);
}
