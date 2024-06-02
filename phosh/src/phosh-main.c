/*
 * Copyright (C) 2024 The Phosh Authors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "phosh-main.h"
#include "phosh-resources.h"

#include <handy.h>
#include <call-ui.h>

void
phosh_init (void)
{
  static gsize resources_initialized = FALSE;

  if (g_once_init_enter (&resources_initialized)) {
    phosh_register_resource ();
    g_once_init_leave (&resources_initialized, TRUE);
  }

  hdy_init ();
  cui_init (TRUE);
}

void
phosh_uninit (void)
{
  cui_uninit ();
}
