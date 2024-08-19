/*
 * Copyright (C) 2022-2023 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-config.h"

#include "gm-device-info.h"
#include "gm-display-panel.h"

#define GM_RESOURCE_PREFIX "/org/gnome/gmobile/"
#define GM_DISPLAY_PANEL_RESOURCE_PREFIX GM_RESOURCE_PREFIX "devices/display-panels/"

/**
 * GmDeviceInfo:
 *
 * Get device dependent information.
 *
 * Allows to query device dependent information from different
 * sources (currently we only look a the built-in gresources).
 *
 * The lookups are currently based on device tree compatibles.
 * See [func@device_tree_get_compatibles].
 *
 * Since: 0.0.1
 */

enum {
  PROP_0,
  PROP_COMPATIBLES,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];

struct _GmDeviceInfo {
  GObject         parent;

  GStrv           compatibles;
  GmDisplayPanel *panel;
};
G_DEFINE_TYPE (GmDeviceInfo, gm_device_info, G_TYPE_OBJECT)


static void
gm_device_info_set_property (GObject      *object,
                             guint         property_id,
                             const GValue *value,
                             GParamSpec   *pspec)
{
  GmDeviceInfo *self = GM_DEVICE_INFO (object);

  switch (property_id) {
  case PROP_COMPATIBLES:
    g_strfreev (self->compatibles);
    self->compatibles = g_value_dup_boxed (value);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
gm_device_info_get_property (GObject    *object,
                             guint       property_id,
                             GValue     *value,
                             GParamSpec *pspec)
{
  GmDeviceInfo *self = GM_DEVICE_INFO (object);

  switch (property_id) {
  case PROP_COMPATIBLES:
    g_value_set_boxed (value, self->compatibles);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
gm_device_info_finalize (GObject *object)
{
  GmDeviceInfo *self = GM_DEVICE_INFO(object);

  g_clear_object (&self->panel);
  g_clear_pointer (&self->compatibles, g_strfreev);

  G_OBJECT_CLASS (gm_device_info_parent_class)->finalize (object);
}


static void
gm_device_info_class_init (GmDeviceInfoClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = gm_device_info_get_property;
  object_class->set_property = gm_device_info_set_property;
  object_class->finalize = gm_device_info_finalize;

  /**
   * GmDeviceInfo:compatibles:
   *
   * The compatibles to look up device information for.
   *
   * Since: 0.0.1
   */
  props[PROP_COMPATIBLES] =
    g_param_spec_boxed ("compatibles", "", "",
                        G_TYPE_STRV,
                        G_PARAM_READWRITE | G_PARAM_CONSTRUCT_ONLY | G_PARAM_STATIC_STRINGS);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
}


static void
gm_device_info_init (GmDeviceInfo *self)
{
}

/**
 * gm_device_info_new:
 * @compatibles: device tree compatibles
 *
 * Gets device information based on the passed in device tree compatibles.
 *
 * Returns: The known device information
 *
 * Since: 0.0.1
 */
GmDeviceInfo *
gm_device_info_new (const char * const *compatibles)
{
  return GM_DEVICE_INFO (g_object_new (GM_TYPE_DEVICE_INFO,
                                       "compatibles", compatibles,
                                       NULL));
}

/**
 * gm_device_info_get_display_panel:
 * @self: The device info
 *
 * Gets display panel information. Queries the database for the best
 * matching panel based on the device's compatibles.
 *
 * Returns:(transfer none): The display panel information
 *
 * Since: 0.0.1
 */
GmDisplayPanel *
gm_device_info_get_display_panel (GmDeviceInfo *self)
{
  GmDisplayPanel *panel = NULL;

  g_return_val_if_fail (GM_IS_DEVICE_INFO (self), NULL);
  g_return_val_if_fail (self->compatibles, NULL);

  if (self->panel)
    return self->panel;

  for (int i = 0; self->compatibles[i] != NULL; i++) {
    g_autofree char *filename = g_strdup_printf ("%s.json", self->compatibles[i]);
    g_autofree char *resource = NULL;

    resource = g_build_path ("/", GM_DISPLAY_PANEL_RESOURCE_PREFIX, filename, NULL);
    panel = gm_display_panel_new_from_resource (resource, NULL);
    if (panel) {
      self->panel = panel;
      break;
    }
  }

  return self->panel;
}
