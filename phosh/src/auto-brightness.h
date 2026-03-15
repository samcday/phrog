/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "backlight.h"

#include <glib-object.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_AUTO_BRIGHTNESS (phosh_auto_brightness_get_type ())
G_DECLARE_INTERFACE (PhoshAutoBrightness, phosh_auto_brightness, PHOSH, AUTO_BRIGHTNESS, GObject)

/**
 * PhoshAutobrightness:
 * @parent_iface: The parent interface
 * @add_ambient_level: Add a new value received by the ambient light sensor
 *
 * Interface implementations are required to implement all virtual functions.
 */

struct _PhoshAutoBrightnessInterface
{
  GTypeInterface parent_iface;

  void            (*add_ambient_level) (PhoshAutoBrightness *self, double value);
  double          (*get_brightness)    (PhoshAutoBrightness *self);
  PhoshBacklight *(*get_backlight)     (PhoshAutoBrightness *self);
};

void            phosh_auto_brightness_add_ambient_level (PhoshAutoBrightness *self, double level);
double          phosh_auto_brightness_get_brightness (PhoshAutoBrightness *self);
PhoshBacklight *phosh_auto_brightness_get_backlight (PhoshAutoBrightness *self);
