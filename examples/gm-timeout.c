/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

/* Wait for n seconds taking suspend/resume into account */


#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"

#include <glib/gprintf.h>


static void
print_version (void)
{
  g_printf ("gm-timeout %s - Wait a bit\n", GM_VERSION);
  exit (0);
}


static void
print_now (void)
{
  g_autoptr (GDateTime) now = g_date_time_new_now_local ();

  g_message ("Now %.2d:%.2d:%.2d",
	     g_date_time_get_hour (now),
	     g_date_time_get_minute (now),
	     g_date_time_get_second (now));
}


static void
on_timeout (gpointer data)
{
  g_main_loop_quit (data);

  g_message ("Exiting main loop");
}


int main (int argc, char **argv)
{
  g_autoptr(GOptionContext) opt_context = NULL;
  g_autoptr (GMainLoop) loop = NULL;
  g_autoptr (GError) err = NULL;
  int seconds = 60;
  gboolean version = FALSE;

  const GOptionEntry options [] = {
    {"seconds", 's', 0, G_OPTION_ARG_INT, &seconds,
     "Sleep for that many seconds", NULL},
    {"version", 0, 0, G_OPTION_ARG_NONE, &version,
     "Show version information", NULL},
    { NULL, 0, 0, G_OPTION_ARG_NONE, NULL, NULL, NULL }
  };

  opt_context = g_option_context_new ("- Wait a bit");
  g_option_context_add_main_entries (opt_context, options, NULL);
  if (!g_option_context_parse (opt_context, &argc, &argv, &err)) {
    g_warning ("%s", err->message);
    g_clear_error (&err);
    return 1;
  }

  if (version)
    print_version ();

  loop = g_main_loop_new (NULL, FALSE);

  g_message ("Arming timer with %d seconds", seconds);
  gm_timeout_add_seconds_once (seconds, on_timeout, loop);

  print_now ();
  g_main_loop_run (loop);

  print_now ();
}
