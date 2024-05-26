/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */

#pragma once

#include "gm-rect.h"

#include <glib-object.h>

G_BEGIN_DECLS

#define GM_TYPE_CUTOUT (gm_cutout_get_type ())

G_DECLARE_FINAL_TYPE (GmCutout, gm_cutout, GM, CUTOUT, GObject)

GmCutout      *gm_cutout_new (const char *path);
const char    *gm_cutout_get_name (GmCutout *self);
const char    *gm_cutout_get_path (GmCutout *self);
const GmRect  *gm_cutout_get_bounds (GmCutout *self);

G_END_DECLS
