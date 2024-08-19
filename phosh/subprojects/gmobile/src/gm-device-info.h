/*
 * Copyright (C) 2022-2023 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#pragma once

#include "gm-display-panel.h"

#include <glib-object.h>

G_BEGIN_DECLS

#define GM_TYPE_DEVICE_INFO (gm_device_info_get_type ())

G_DECLARE_FINAL_TYPE (GmDeviceInfo, gm_device_info, GM, DEVICE_INFO, GObject)

GmDeviceInfo    *gm_device_info_new (const char * const *compatibles);
GmDisplayPanel  *gm_device_info_get_display_panel (GmDeviceInfo *self);

G_END_DECLS
