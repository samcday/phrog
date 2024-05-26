/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"

#include "gio/gio.h"

static void
test_gm_device_tree_get_compatibles (void)
{
  g_auto (GStrv) compatibles = NULL;
  GError *err = NULL;

  /* nonexistent */
  compatibles = gm_device_tree_get_compatibles (TEST_DATA_DIR "/doesnotexist", &err);
  g_assert_null (compatibles);
  g_assert_error (err, G_IO_ERROR, G_IO_ERROR_NOT_FOUND);
  g_clear_error (&err);

  /* nonexistent, don't store error */
  compatibles = gm_device_tree_get_compatibles (TEST_DATA_DIR "/doesnotexist", NULL);
  g_assert_null (compatibles);

  /* Regular format */
  compatibles = gm_device_tree_get_compatibles (TEST_DATA_DIR "/compatibles1", &err);
  g_assert_no_error (err);
  g_assert_nonnull (compatibles);
  g_assert_cmpstr (compatibles[0], ==, "purism,librem5r4");
  g_assert_cmpstr (compatibles[1], ==, "purism,librem5");
  g_assert_cmpstr (compatibles[2], ==, "fsl,imx8mq");
  g_assert_null (compatibles[3]);
  g_strfreev (compatibles);

  /* empty file */
  compatibles = gm_device_tree_get_compatibles (TEST_DATA_DIR "/compatibles2", &err);
  g_assert_no_error (err);
  g_assert_nonnull (compatibles);
}


gint
main (gint argc, gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/device-tree/get-compatibles", test_gm_device_tree_get_compatibles);

  return g_test_run ();
}
