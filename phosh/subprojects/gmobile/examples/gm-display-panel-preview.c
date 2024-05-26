/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"

#include <gio/gio.h>
#include <glib/gprintf.h>

static void
print_version (void)
{
  g_printf ("gm-display-panel-preview %s\n", GM_VERSION);
  exit (0);
}

#define X_OFF 5
#define Y_OFF 5

static char *
build_svg (GmDisplayPanel *panel)
{
  GString *svg = g_string_new ("");
  int xres = gm_display_panel_get_x_res (panel);
  int yres = gm_display_panel_get_y_res (panel);
  int radius = gm_display_panel_get_border_radius (panel);
  GListModel *cutouts;

  g_string_append_printf (svg,
    "    <svg width=\"%d\" height=\"%d\">\n"
    "      <g transform=\"translate(%d,%d)\">\n"
    "        <!-- The panel -->\n"
    "        <path d=\"M0 %d"
                    "  a %d %d 0 0 1 %d %d"
                    "  h%d"
                    "  a %d %d 0 0 1 %d %d"
                    "  v%d"
                    "  a %d %d 0 0 1 %d %d"
                    "  h%d"
                    "  a %d %d 0 0 1 %d %d"
                    "  Z \""
                    " stroke=\"black\" stroke-width=\"2\" fill=\"lightgrey\" />\n",
                          xres + 2 * X_OFF, yres + 2 * Y_OFF,
                          X_OFF, Y_OFF,
                          radius,
                          radius, radius, radius, -radius,
                          xres - 2 * radius,
                          radius, radius, radius, radius,
                          yres - 2 * radius,
                          radius, radius, -radius, radius,
                          -xres + 2 * radius,
                          radius, radius, -radius, -radius);

  cutouts = gm_display_panel_get_cutouts (panel);
  for (int i = 0; i < g_list_model_get_n_items (cutouts); i++) {
    g_autoptr (GmCutout) cutout = g_list_model_get_item (cutouts, i);
    if (cutout) {
      const GmRect *bounds = gm_cutout_get_bounds (cutout);
      const char *name = gm_cutout_get_name (cutout) ?: "";
      const char *cutout_path = gm_cutout_get_path (cutout);

      if (cutout_path == NULL) {
        g_warning ("Failed to get cutout path for '%s' - skipping", name);
        continue;
      }

      g_string_append_printf (svg,
    "        <!-- cutout %s -->\n"
    "        <path d=\"%s\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n",
                              name,
                              cutout_path);
      g_string_append_printf (svg,
    "        <!-- bbox of cutout %s -->\n"
    "        <rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"%d\""
                        " fill=\"red\" fill-opacity=\"0.1\" />\n",
                              name,
                              bounds->x, bounds->y, bounds->width, bounds->height);
      }
  }

  g_string_append_printf (svg,
     "      </g>\n"
     "    </svg>\n");

  return g_string_free (svg, FALSE);
}


static char *
build_html (GmDisplayPanel *panel)
{
  GString *html = g_string_new ("");
  g_autofree char *svg = NULL;

  svg = build_svg (panel);
  g_string_append_printf (html,
    "<!DOCTYPE html>\n"
    "<html>\n"
    "  <head>\n"
    "    <title>%s</title>\n"
    "    <meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />\n"
    "  </head>\n"
    "  <body>\n"
    "%s"
    "  </body>\n"
    "</html>", gm_display_panel_get_name (panel), svg);

  return g_string_free (html, FALSE);
}


int main (int argc, char **argv)
{
  g_autoptr (GOptionContext) opt_context = NULL;
  gboolean version = FALSE;
  gboolean svg = FALSE;
  const char *output_file = NULL;
  g_autofree char *content = NULL;
  g_autoptr (GError) err = NULL;
  g_autoptr (GmDeviceInfo) info = NULL;
  g_auto (GStrv) compatibles = NULL;
  GmDisplayPanel *panel = NULL;
  GStrv compatibles_opt = NULL;

  const GOptionEntry options [] = {
    {"compatible", 'c', 0, G_OPTION_ARG_STRING_ARRAY, &compatibles_opt,
     "Device tree compatibles to use for panel lookup ", NULL},
    {"output", 'o', 0, G_OPTION_ARG_STRING, &output_file,
     "The output file name", NULL},
    {"svg", 's', 0, G_OPTION_ARG_NONE, &svg,
     "Output svg instead of html", NULL},
    {"version", 0, 0, G_OPTION_ARG_NONE, &version,
     "Show version information", NULL},
    { NULL, 0, 0, G_OPTION_ARG_NONE, NULL, NULL, NULL }
  };

  opt_context = g_option_context_new ("- panel preview");
  g_option_context_add_main_entries (opt_context, options, NULL);
  if (!g_option_context_parse (opt_context, &argc, &argv, &err)) {
    g_warning ("%s", err->message);
    g_clear_error (&err);
    return EXIT_FAILURE;
  }

  if (version)
    print_version ();

  if (compatibles_opt && compatibles_opt[0]) {
    compatibles = g_strdupv (compatibles_opt);
  } else {
    compatibles = gm_device_tree_get_compatibles (NULL, &err);
    if (compatibles == NULL) {
      g_critical ("Failed to get compatibles: %s", err->message);
      return EXIT_FAILURE;
    }
  }

  info = gm_device_info_new ((const char *const *)compatibles);
  panel = gm_device_info_get_display_panel (info);
  if (panel == NULL) {
    g_critical ("Failed to find any panel");
    return EXIT_FAILURE;
  }

  if (svg)
    content = build_svg (panel);
  else
    content = build_html (panel);

  if (output_file) {
    g_autoptr (GFile) dest = g_file_new_for_path (output_file);

    if (!g_file_replace_contents (dest,
                                  content,
                                  strlen (content),
                                  NULL, FALSE,
                                  G_FILE_CREATE_REPLACE_DESTINATION,
                                  NULL, NULL,
                                  &err)) {
      g_critical ("Failed to write html: %s", err->message);
      return EXIT_FAILURE;
    }
  } else {
    g_printf ("%s", content);
  }

  return EXIT_SUCCESS;
}
