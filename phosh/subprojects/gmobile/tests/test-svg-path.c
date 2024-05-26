/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define GMOBILE_USE_UNSTABLE_API
#include "gm-svg-path.h"

#include "gio/gio.h"

static void
test_gm_svg_path_get_bounding_box_abs (void)
{
  const char *path = "M455, 0 \
     V 79    \
     H 625   \
     V 0     \
     Z ";
  gboolean success;
  int x1, x2, y1, y2;
  g_autoptr (GError) err = NULL;

  success = gm_svg_path_get_bounding_box (path, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 455);
  g_assert_cmpint (x2, ==, 625);
  g_assert_cmpint (y1, ==, 0);
  g_assert_cmpint (y2, ==, 79);
}


static void
test_gm_svg_path_get_bounding_box_rel (void)
{
  const char *path = "m 455 0 \
     v 79    \
     h 170   \
     v 0     \
     Z";
  gboolean success;
  int x1, x2, y1, y2;
  g_autoptr (GError) err = NULL;

  success = gm_svg_path_get_bounding_box (path, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 455);
  g_assert_cmpint (x2, ==, 625);
  g_assert_cmpint (y1, ==, 0);
  g_assert_cmpint (y2, ==, 79);
}


static void
test_gm_svg_path_get_bounding_box_arc (void)
{
  const char *path = "M 10 315\n"
                     "L 110 215\n"
                     "A 30 50 0 0 1 162 160";
  gboolean success;
  int x1, x2, y1, y2;
  g_autoptr (GError) err = NULL;

  success = gm_svg_path_get_bounding_box (path, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 10);
  g_assert_cmpint (x2, ==, 162);
  g_assert_cmpint (y1, ==, 136);
  g_assert_cmpint (y2, ==, 315);
}


static void
test_gm_svg_path_get_bounding_box_quad_bezier (void)
{
  /* control point not between start and end */
  const char *path1 = "M 70 250 Q 20 110 220 60 Z";
  /* same but in relative */
  const char *path1_rel = "M 70 250 q -50 -140 150 -190 Z";
  /* control point between start and end */
  const char *path2 = "M 70 250 Q 110 110 220 60 Z";
  gboolean success;
  int x1, x2, y1, y2;
  g_autoptr (GError) err = NULL;

  success = gm_svg_path_get_bounding_box (path1, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 60);
  g_assert_cmpint (x2, ==, 220);
  g_assert_cmpint (y1, ==, 60);
  g_assert_cmpint (y2, ==, 250);

  success = gm_svg_path_get_bounding_box (path1_rel, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 60);
  g_assert_cmpint (x2, ==, 220);
  g_assert_cmpint (y1, ==, 60);
  g_assert_cmpint (y2, ==, 250);

  success = gm_svg_path_get_bounding_box (path2, &x1, &x2, &y1, &y2, &err);
  g_assert_no_error (err);
  g_assert_true (success);

  g_assert_cmpint (x1, ==, 70);
  g_assert_cmpint (x2, ==, 220);
  g_assert_cmpint (y1, ==, 60);
  g_assert_cmpint (y2, ==, 250);
}


gint
main (gint argc, gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/svg-path/bounding_box/abs", test_gm_svg_path_get_bounding_box_abs);
  g_test_add_func ("/Gm/svg-path/bounding_box/rel", test_gm_svg_path_get_bounding_box_rel);
  g_test_add_func ("/Gm/svg-path/bounding_box/arc", test_gm_svg_path_get_bounding_box_arc);
  g_test_add_func ("/Gm/svg-path/bounding_box/quad_bezier",
                   test_gm_svg_path_get_bounding_box_quad_bezier);

  return g_test_run ();
}
