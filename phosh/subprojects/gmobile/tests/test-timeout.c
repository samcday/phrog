/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"


static void
on_timeout (gpointer data)
{
  g_main_loop_quit (data);
}


static void
test_gm_timeout_simple (void)
{
  g_autoptr (GMainLoop) loop = NULL;
  int seconds = 1;

  loop = g_main_loop_new (NULL, FALSE);
  gm_timeout_add_seconds_once (seconds, on_timeout, loop);
  g_main_loop_run (loop);
}


static void
on_timeout2 (gpointer data)
{
  g_assert_not_reached ();
}


static gboolean
remove_timeout (gpointer data)
{
  g_source_remove (GPOINTER_TO_UINT (data));

  return G_SOURCE_REMOVE;
}


static void
test_gm_timeout_remove (void)
{
  g_autoptr (GMainLoop) loop = NULL;
  guint id;

  loop = g_main_loop_new (NULL, FALSE);
  id = gm_timeout_add_seconds_once (1, on_timeout2, NULL);
  g_assert_nonnull (g_main_context_find_source_by_id (NULL, id));
  g_idle_add (remove_timeout, GUINT_TO_POINTER (id));
  /* End the main loop, id must not have fired yet */
  g_timeout_add_seconds (2, (GSourceFunc)on_timeout, loop);
  g_main_loop_run (loop);
  /* source got removed */
  g_assert_null (g_main_context_find_source_by_id (NULL, id));
}


gint
main (gint argc,
      gchar *argv[])
{
  g_test_init (&argc, &argv, NULL);

  g_test_add_func ("/Gm/timeout/simple", test_gm_timeout_simple);
  g_test_add_func ("/Gm/timeout/remove", test_gm_timeout_remove);

  return g_test_run ();
}
