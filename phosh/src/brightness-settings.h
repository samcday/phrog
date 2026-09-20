/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_BRIGHTNESS_SETTINGS phosh_brightness_settings_get_type ()

G_DECLARE_FINAL_TYPE (PhoshBrightnessSettings, phosh_brightness_settings, PHOSH,
                      BRIGHTNESS_SETTINGS, GtkBin)

GtkWidget *phosh_brightness_settings_new (void);
void       phosh_brightness_settings_hide_details (PhoshBrightnessSettings *self);

G_END_DECLS
