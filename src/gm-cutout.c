/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-cutout.h"
#include "gm-rect.h"
#include "gm-svg-path.h"

#include <gio/gio.h>

/**
 * GmCutout:
 *
 * A display cutout or notch.
 *
 * A display cutout is an area of a display that is not usable for
 * rendering e.g.  because a camera or speaker is placed there. This
 * includes so called "notches".  The are needs to be avoided when
 * rendering. It is described by a SVG path. Each cutouts coordinate
 * systems is located at the top left display corner in the displays
 * natural orientation. Applications can query the area to avoid
 * for rendering via the `bounds` property so they don't need to
 * deal with the SVG path themselves.
 *
 * Since: 0.0.2
 */

enum {
  PROP_0,
  PROP_NAME,
  PROP_PATH,
  PROP_BOUNDS,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];

struct _GmCutout {
  GObject   parent;

  char     *name;
  char     *path;
  GmRect    bounds;
};

G_DEFINE_TYPE (GmCutout, gm_cutout, G_TYPE_OBJECT);

static gboolean
gm_cutout_set_path (GmCutout *self, const char *path, GError **err)
{
  g_autoptr (GError) local_err = NULL;
  int x1, y1, x2, y2;

  if (g_strcmp0 (self->path, path) == 0)
    return TRUE;

  if (gm_svg_path_get_bounding_box (path, &x1, &x2, &y1, &y2, &local_err) == FALSE) {
    if (err)
      *err = g_error_copy (local_err);
    /* Tracking errors when setting properties can be hard so make it
     * simple to get some debugging */
    g_debug ("Failed to parse bounding box for %s: %s", path, local_err->message);
    return FALSE;
  }

  g_free (self->path);
  self->path = g_strdup (path);

  self->bounds.x = x1;
  self->bounds.y = y1;
  self->bounds.width = x2 - x1;
  self->bounds.height = y2 - y1;

  return TRUE;
}


static void
gm_cutout_set_property (GObject      *object,
                        guint         property_id,
                        const GValue *value,
                        GParamSpec   *pspec)
{
  GmCutout *self = GM_CUTOUT (object);

  switch (property_id) {
  case PROP_NAME:
    g_free (self->name);
    self->name = g_value_dup_string (value);
    break;
  case PROP_PATH:
    gm_cutout_set_path (self, g_value_get_string (value), NULL);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
gm_cutout_get_property (GObject    *object,
                        guint       property_id,
                        GValue     *value,
                        GParamSpec *pspec)
{
  GmCutout *self = GM_CUTOUT (object);

  switch (property_id) {
  case PROP_NAME:
    g_value_set_string (value, gm_cutout_get_name (self));
    break;
  case PROP_PATH:
    g_value_set_string (value, gm_cutout_get_path (self));
    break;
  case PROP_BOUNDS:
    g_value_set_boxed (value, gm_cutout_get_bounds (self));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
gm_cutout_finalize (GObject *object)
{
  GmCutout *self = GM_CUTOUT (object);

  g_clear_pointer (&self->name, g_free);
  g_clear_pointer (&self->path, g_free);

  G_OBJECT_CLASS (gm_cutout_parent_class)->finalize (object);
}


static void
gm_cutout_class_init (GmCutoutClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = gm_cutout_get_property;
  object_class->set_property = gm_cutout_set_property;
  object_class->finalize = gm_cutout_finalize;

  /**
   * GmCutout:name:
   *
   * A name identifying the cutout.
   *
   * Since: 0.0.2
   */
  props[PROP_NAME] =
    g_param_spec_string ("name", "", "",
                         NULL,
                         G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS | G_PARAM_CONSTRUCT_ONLY);
  /**
   * GmCutout:path:
   *
   * The SVG path that describes the display cutout or notch.
   *
   * Since: 0.0.2
   */
  props[PROP_PATH] =
    g_param_spec_string ("path", "", "",
                         NULL,
                         G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS | G_PARAM_CONSTRUCT_ONLY);
  /**
   * GmCutout:bounds:
   *
   * Rectangular bounds of the cutout
   *
   * Since: 0.0.2
   */
  props[PROP_BOUNDS] =
    g_param_spec_boxed ("bounds", "", "",
                        GM_TYPE_RECT,
                        G_PARAM_READABLE | G_PARAM_STATIC_STRINGS);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
}


static void
gm_cutout_init (GmCutout *self)
{
}

/**
 * gm_cutout_new:
 * @path: An svg path
 *
 * Create a new cutout based on the given SVG path.
 *
 * Returns: The cutout.
 *
 * Since: 0.0.2
 */
GmCutout *
gm_cutout_new (const char *path)
{
  return GM_CUTOUT (g_object_new (GM_TYPE_CUTOUT, "path", path, NULL));
}

/**
 * gm_cutout_get_name:
 * @self: A cutout
 *
 * The name of the cutout.
 *
 * Returns: The cutout's name.
 *
 * Since: 0.0.2
 */
const char *
gm_cutout_get_name (GmCutout *self)
{
  g_return_val_if_fail (GM_IS_CUTOUT (self), NULL);

  return self->name;
}

/**
 * gm_cutout_get_path:
 * @self: A cutout
 *
 * Gets the SVG path describing the shape of the cutout.
 *
 * Returns: The cutout's shape as SVG path
 *
 * Since: 0.0.2
 */
const char *
gm_cutout_get_path (GmCutout *self)
{
  g_return_val_if_fail (GM_IS_CUTOUT (self), NULL);

  return self->path;
}

/**
 * gm_cutout_get_bounds:
 * @self: A cutout
 *
 * Gets the bounding box of the cutout.
 *
 * Returns: The bounding box.
 *
 * Since: 0.0.2
 */
const GmRect *
gm_cutout_get_bounds (GmCutout *self)
{
  g_return_val_if_fail (GM_IS_CUTOUT (self), NULL);

  return &self->bounds;
}
