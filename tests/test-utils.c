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
test_gm_str_is_null_or_empty (void)
{
  g_assert_true (gm_str_is_null_or_empty ((char *)NULL));
  g_assert_true (gm_str_is_null_or_empty (""));
  g_assert_false (gm_str_is_null_or_empty ("notempty"));
}


static void
test_gm_strv_is_null_or_empty (void)
{
  const char * const *not_empty = (const char *const []) {"still", "not", "empty", NULL };

  g_assert_true (gm_strv_is_null_or_empty ((char **)NULL));
  g_assert_true (gm_strv_is_null_or_empty ((char *[]){NULL}));
  g_assert_false (gm_strv_is_null_or_empty (((char *[]){"notempty", NULL,})));
  g_assert_cmpint (g_strv_length ((GStrv)not_empty), ==, 3);
  /* Check that we don't hit any compiler warnings */
  g_assert_false (gm_strv_is_null_or_empty (not_empty));
  g_assert_false (gm_strv_is_null_or_empty ((GStrv)not_empty));
}


gint main (gint argc, gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/util/str_null_or_empty", test_gm_str_is_null_or_empty);
  g_test_add_func ("/Gm/util/strv_null_or_empty", test_gm_strv_is_null_or_empty);

  return g_test_run ();
}
