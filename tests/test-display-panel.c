/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"


static void
test_gm_display_panel_parse (void)
{
  const char *json = "                                "
    "{                                                "
    " \"name\": \"Oneplus 6T\",                       "
    " \"x-res\": 1080,                                "
    " \"y-res\": 2340,                                "
    " \"border-radius\": 10,                          "
    " \"width\": 68,                                  "
    " \"height\": 145,                                "
    " \"cutouts\" : [                                 "
    "     {                                           "
    "        \"name\": \"notch\",                     "
    "        \"path\": \"M 455 0 V 79 H 625 V 0 Z\"   "
    "     }                                           "
    "  ]                                              "
    "}                                                ";
  g_autoptr (GError) err = NULL;
  g_autoptr (GmDisplayPanel) panel = NULL;
  g_autoptr (GmCutout) cutout = NULL;
  GListModel *cutouts;
  const GmRect *bounds;

  panel = gm_display_panel_new_from_data (json, &err);
  g_assert_no_error (err);
  g_assert_nonnull (panel);

  cutouts = gm_display_panel_get_cutouts (panel);
  g_assert_cmpint (g_list_model_get_n_items (cutouts), ==, 1);
  cutout = g_list_model_get_item (cutouts, 0);
  g_assert_nonnull (cutout);
  g_assert_cmpstr (gm_cutout_get_name (cutout), ==, "notch");
  g_assert_cmpstr (gm_cutout_get_path (cutout), ==, "M 455 0 V 79 H 625 V 0 Z");
  bounds = gm_cutout_get_bounds (cutout);
  g_assert_cmpint (bounds->x, ==, 455);
  g_assert_cmpint (bounds->y, ==, 0);
  g_assert_cmpint (bounds->width, ==, 170);
  g_assert_cmpint (bounds->height, ==, 79);

  g_assert_cmpint (gm_display_panel_get_x_res (panel), ==, 1080);
  g_assert_cmpint (gm_display_panel_get_y_res (panel), ==, 2340);

  g_assert_cmpint (gm_display_panel_get_border_radius (panel), ==, 10);

  g_assert_cmpint (gm_display_panel_get_width (panel), ==, 68);
  g_assert_cmpint (gm_display_panel_get_height (panel), ==, 145);
}


gint
main (gint argc, gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/display-panel/parse", test_gm_display_panel_parse);

  return g_test_run ();
}
