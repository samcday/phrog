/*
 * Copyright (C) 2024 The Phosh Authors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include "phosh-main.h"
#include <handy.h>
#include <call-ui.h>

void
phosh_init (void)
{
  hdy_init ();
  cui_init (TRUE);
}

void
phosh_uninit (void)
{
  cui_uninit ();
}
