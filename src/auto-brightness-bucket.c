/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define G_LOG_DOMAIN "phosh-auto-brightness-bucket"

#include "phosh-config.h"

#include "auto-brightness-bucket.h"

/**
 * PhoshAutoBrightnessBucket:
 *
 * Auto brightness handling using a bucket approach
 */

enum {
  PROP_0,
  PROP_BRIGHTNESS,
  PROP_BACKLIGHT,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];


typedef struct {
  uint min, max;
  double brightness;
} Bucket;


/* See https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/sensors-adaptive-brightness */
static Bucket buckets[] = {
  {    0,    10, 0.10 },
  {    5,    50, 0.25 },
  {   15,   100, 0.40 },
  {   60,   300, 0.55 },
  {  150,   400, 0.70 },
  {  250,   650, 0.85 },
  {  350,  2000, 1.00 },
  { 1000,  7000, 1.15 },
  { 5000, 10000, 1.30 },
};


struct _PhoshAutoBrightnessBucket {
  GObject               parent;

  PhoshBacklight       *backlight;
  double                brightness;
  uint                  index;
};


static void auto_brightness_interface_init (PhoshAutoBrightnessInterface *iface);

G_DEFINE_TYPE_WITH_CODE (PhoshAutoBrightnessBucket, phosh_auto_brightness_bucket, G_TYPE_OBJECT,
                         G_IMPLEMENT_INTERFACE (PHOSH_TYPE_AUTO_BRIGHTNESS,
                                                auto_brightness_interface_init))

static gboolean
is_in_bucket (uint index, double level)
{
  Bucket bucket = buckets[index];

  if (level >= bucket.min && level <= bucket.max)
    return TRUE;

  return FALSE;
}


static void
auto_brightness_bucket_add_ambient_level (PhoshAutoBrightness *auto_brightness, double level)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (auto_brightness);
  uint index;

  if (is_in_bucket (self->index, level))
    return;

  index = self->index;
  if (level < buckets[index].min) {
    for (; index > 0; index--) {
      if (is_in_bucket (index, level))
        break;
    }
  } else {
    for (; index < G_N_ELEMENTS (buckets) - 1; index++) {
      if (is_in_bucket (index, level))
        break;
    }
  }

  self->brightness = buckets[index].brightness;

  if (self->index == index)
    return;

  self->index = index;
  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_BRIGHTNESS]);
}


static double
auto_brightness_bucket_get_brightness (PhoshAutoBrightness *auto_brightness)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (auto_brightness);

  return self->brightness;
}


static PhoshBacklight *
auto_brightness_bucket_get_backlight (PhoshAutoBrightness *auto_brightness)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (auto_brightness);

  return self->backlight;
}


static void
auto_brightness_interface_init (PhoshAutoBrightnessInterface *iface)
{
  iface->add_ambient_level = auto_brightness_bucket_add_ambient_level;
  iface->get_brightness = auto_brightness_bucket_get_brightness;
  iface->get_backlight = auto_brightness_bucket_get_backlight;
}


static void
phosh_auto_brightness_bucket_set_property (GObject      *object,
                                           guint         property_id,
                                           const GValue *value,
                                           GParamSpec   *pspec)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (object);

  switch (property_id) {
  case PROP_BACKLIGHT:
    g_set_object (&self->backlight, g_value_get_object (value));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_auto_brightness_bucket_get_property (GObject    *object,
                                           guint       property_id,
                                           GValue     *value,
                                           GParamSpec *pspec)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (object);

  switch (property_id) {
  case PROP_BACKLIGHT:
    g_value_set_object (value, self->backlight);
    break;
  case PROP_BRIGHTNESS:
    g_value_set_double (value, self->brightness);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_auto_brightness_bucket_dispose (GObject *object)
{
  PhoshAutoBrightnessBucket *self = PHOSH_AUTO_BRIGHTNESS_BUCKET (object);

  g_clear_object (&self->backlight);

  G_OBJECT_CLASS (phosh_auto_brightness_bucket_parent_class)->dispose (object);
}


static void
phosh_auto_brightness_bucket_class_init (PhoshAutoBrightnessBucketClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = phosh_auto_brightness_bucket_get_property;
  object_class->set_property = phosh_auto_brightness_bucket_set_property;
  object_class->dispose = phosh_auto_brightness_bucket_dispose;

  g_object_class_override_property (object_class, PROP_BACKLIGHT, "backlight");
  props[PROP_BACKLIGHT] = g_object_class_find_property (object_class, "backlight");

  g_object_class_override_property (object_class, PROP_BRIGHTNESS, "brightness");
  props[PROP_BRIGHTNESS] = g_object_class_find_property (object_class, "brightness");
}


static void
phosh_auto_brightness_bucket_init (PhoshAutoBrightnessBucket *self)
{
  self->brightness = 0.55;
  self->index = 3;
}


PhoshAutoBrightnessBucket *
phosh_auto_brightness_bucket_new (void)
{
  return g_object_new (PHOSH_TYPE_AUTO_BRIGHTNESS_BUCKET, NULL);
}
