/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "auto-brightness.h"

#include <glib-object.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_AUTO_BRIGHTNESS_BUCKET (phosh_auto_brightness_bucket_get_type ())

G_DECLARE_FINAL_TYPE (PhoshAutoBrightnessBucket, phosh_auto_brightness_bucket,
                      PHOSH, AUTO_BRIGHTNESS_BUCKET, GObject)

PhoshAutoBrightnessBucket *phosh_auto_brightness_bucket_new (void);

G_END_DECLS
