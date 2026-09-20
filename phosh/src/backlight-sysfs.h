/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "backlight-priv.h"

G_BEGIN_DECLS

#define PHOSH_TYPE_BACKLIGHT_SYSFS (phosh_backlight_sysfs_get_type ())

G_DECLARE_FINAL_TYPE (PhoshBacklightSysfs, phosh_backlight_sysfs, PHOSH, BACKLIGHT_SYSFS,
                      PhoshBacklight)

PhoshBacklightSysfs *phosh_backlight_sysfs_new (const char *connector_name, GError **error);

G_END_DECLS
