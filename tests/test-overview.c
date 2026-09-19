/*
 * Copyright (C) 2018 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "overview.h"

#include <adwaita.h>

static void
test_phosh_overview_new(void)
{
  GtkWidget *window = g_object_ref_sink (phosh_overview_new ());
  g_assert (window);
  g_object_unref (window);
}


int
main (int   argc,
      char *argv[])
{
  gtk_test_init (&argc, &argv, NULL);
  adw_init ();

  g_test_add_func("/phosh/overview/new", test_phosh_overview_new);
  return g_test_run();
}
