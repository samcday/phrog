/*
 * Copyright (C) 2022-2023 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-main.h"
#include "gm-resources.h"

/**
 * gm_init:
 *
 * Call this function to initialize the library explicitly. This makes
 * the embedded device information available.
 *
 * Since: 0.0.1
 */
void
gm_init (void)
{
  static gsize initialized = FALSE;

  if (g_once_init_enter (&initialized)) {
    /*
     * gmobile is currently meant as static library so register
     * resources explicitly.  otherwise they get dropped during static
     * linking
     */
    gm_register_resource ();
    g_once_init_leave (&initialized, TRUE);
  }
}
