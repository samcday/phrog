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
test_gm_cutout_bounding_box (void)
{
  char *path = "M455, 0 \
                V 79    \
                H 625   \
                V 0     \
                Z";
  g_autoptr (GError) err = NULL;
  GmCutout *cutout;
  const GmRect *rect;

  cutout = gm_cutout_new (path);
  g_assert_no_error (err);

  rect = gm_cutout_get_bounds (cutout);
  g_assert_cmpint (rect->x, ==, 455);
  g_assert_cmpint (rect->y, ==, 0);
  g_assert_cmpint (rect->width, ==, 170);
  g_assert_cmpint (rect->height, ==, 79);
}


gint
main (gint argc, gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/cutout/bounding-box", test_gm_cutout_bounding_box);

  return g_test_run ();
}
