/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define G_LOG_DOMAIN "phosh-auto-brightness"

#include "auto-brightness.h"

/**
 * PhoshAutoBrightness:
 *
 * Implementations for the automatic brightness algorithm.
 *
 * Since: 0.51.0
 */

G_DEFINE_INTERFACE (PhoshAutoBrightness, phosh_auto_brightness, G_TYPE_OBJECT)

void
phosh_auto_brightness_default_init (PhoshAutoBrightnessInterface *iface)
{
  /**
   * PhoshAutoBacklight:relative-brightness:
   *
   * The relative brightness the given backlight should use
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_double ("brightness", "", "",
                         0.0, 1.0, 0.55,
                         G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY | G_PARAM_STATIC_STRINGS));
  /**
   * PhoshAutoBacklight:backlight:
   *
   * The backlight this auto brightness handler operates on
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_object ("backlight", "", "",
                         PHOSH_TYPE_BACKLIGHT,
                         G_PARAM_READWRITE | G_PARAM_CONSTRUCT_ONLY | G_PARAM_STATIC_STRINGS));
}


void
phosh_auto_brightness_add_ambient_level (PhoshAutoBrightness *self, double level)
{
  PhoshAutoBrightnessInterface *iface;

  g_return_if_fail (PHOSH_IS_AUTO_BRIGHTNESS (self));

  iface = PHOSH_AUTO_BRIGHTNESS_GET_IFACE (self);
  iface->add_ambient_level (self, level);
}


double
phosh_auto_brightness_get_brightness (PhoshAutoBrightness *self)
{
  PhoshAutoBrightnessInterface *iface;

  iface = PHOSH_AUTO_BRIGHTNESS_GET_IFACE (self);
  return iface->get_brightness (self);
}


PhoshBacklight *
phosh_auto_brightness_get_backlight (PhoshAutoBrightness *self)
{
  PhoshAutoBrightnessInterface *iface;

  iface = PHOSH_AUTO_BRIGHTNESS_GET_IFACE (self);
  return iface->get_backlight (self);
}
