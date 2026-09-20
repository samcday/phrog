/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "phosh-dbus-debug-control.h"

#include <glib-object.h>

G_BEGIN_DECLS



#define PHOSH_TYPE_DEBUG_CONTROL (phosh_debug_control_get_type ())

G_DECLARE_FINAL_TYPE (PhoshDebugControl, phosh_debug_control, PHOSH, DEBUG_CONTROL,
                      PhoshDBusDebugControlSkeleton)

PhoshDebugControl       *phosh_debug_control_new (void);
void                    phosh_debug_control_set_exported (PhoshDebugControl *self,
                                                          gboolean           exported);

G_END_DECLS
