/*
 * Copyright (C) 2022-2023 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-cutout.h"
#include "gm-display-panel.h"
#include "gm-main.h"

#include <json-glib/json-glib.h>

/**
 * GmDisplayPanel:
 *
 * Physical properties of a display panel like size, cutouts and
 * rounded corners.
 *
 * Since: 0.0.1
 */

enum {
  PROP_0,
  PROP_NAME,
  PROP_CUTOUTS,
  PROP_X_RES,
  PROP_Y_RES,
  PROP_WIDTH,
  PROP_HEIGHT,
  PROP_BORDER_RADIUS,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];

struct _GmDisplayPanel {
  GObject     parent;

  char       *name;
  GListStore *cutouts;
  int         x_res;
  int         y_res;
  int         border_radius;
  int         width;
  int         height;
};

static void gm_display_panel_json_serializable_iface_init (JsonSerializableIface *iface);

G_DEFINE_TYPE_WITH_CODE (GmDisplayPanel, gm_display_panel, G_TYPE_OBJECT,
                         G_IMPLEMENT_INTERFACE (JSON_TYPE_SERIALIZABLE,
                                                gm_display_panel_json_serializable_iface_init));

static void
gm_display_panel_set_property (GObject      *object,
                               guint         property_id,
                               const GValue *value,
                               GParamSpec   *pspec)
{
  GmDisplayPanel *self = GM_DISPLAY_PANEL (object);

  switch (property_id) {
  case PROP_NAME:
    g_free (self->name);
    self->name = g_value_dup_string (value);
    break;
  case PROP_CUTOUTS:
    g_set_object (&self->cutouts, g_value_get_object (value));
    break;
  case PROP_X_RES:
    self->x_res = g_value_get_int (value);
    break;
  case PROP_Y_RES:
    self->y_res = g_value_get_int (value);
    break;
  case PROP_BORDER_RADIUS:
    self->border_radius = g_value_get_int (value);
    break;
  case PROP_WIDTH:
    self->width = g_value_get_int (value);
    break;
  case PROP_HEIGHT:
    self->height = g_value_get_int (value);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
gm_display_panel_get_property (GObject    *object,
                               guint       property_id,
                               GValue     *value,
                               GParamSpec *pspec)
{
  GmDisplayPanel *self = GM_DISPLAY_PANEL (object);

  switch (property_id) {
  case PROP_NAME:
    g_value_set_string (value, self->name);
    break;
  case PROP_CUTOUTS:
    g_value_set_object (value, self->cutouts);
    break;
  case PROP_X_RES:
    g_value_set_int (value, gm_display_panel_get_x_res (self));
    break;
  case PROP_Y_RES:
    g_value_set_int (value, gm_display_panel_get_y_res (self));
    break;
  case PROP_BORDER_RADIUS:
    g_value_set_int (value, gm_display_panel_get_border_radius (self));
    break;
  case PROP_WIDTH:
    g_value_set_int (value, gm_display_panel_get_width (self));
    break;
  case PROP_HEIGHT:
    g_value_set_int (value, gm_display_panel_get_height (self));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static JsonNode *
gm_display_panel_serializable_serialize_property (JsonSerializable *serializable,
                                                  const gchar      *property_name,
                                                  const GValue     *value,
                                                  GParamSpec       *pspec)
{
  GmDisplayPanel *self = GM_DISPLAY_PANEL (serializable);
  JsonNode *node = NULL;

  if (g_strcmp0 (property_name, "cutouts") == 0) {
    JsonArray *array = json_array_sized_new (1);

    for (int i = 0; i < g_list_model_get_n_items (G_LIST_MODEL (self->cutouts)); i++) {
      g_autoptr (GObject) cutout = g_list_model_get_item (G_LIST_MODEL (self->cutouts), i);
      json_array_add_element (array, json_gobject_serialize (cutout));
    }
    node = json_node_init_array (json_node_alloc (), array);
  } else {
    node = json_serializable_default_serialize_property (serializable,
                                                         property_name,
                                                         value,
                                                         pspec);
  }
  return node;
}


static gboolean
gm_display_panel_serializable_deserialize_property (JsonSerializable *serializable,
                                                    const gchar      *property_name,
                                                    GValue           *value,
                                                    GParamSpec       *pspec,
                                                    JsonNode         *property_node)
{
  if (g_strcmp0 (property_name, "cutouts") == 0) {
    if (JSON_NODE_TYPE (property_node) == JSON_NODE_NULL) {
      g_value_set_pointer (value, NULL);
      return TRUE;
    } else if (JSON_NODE_TYPE (property_node) == JSON_NODE_ARRAY) {
      JsonArray *array = json_node_get_array (property_node);
      guint array_len = json_array_get_length (array);
      GListStore *cutouts = g_list_store_new (GM_TYPE_CUTOUT);

      for (int i = 0; i < array_len; i++) {
        JsonNode *element_node = json_array_get_element (array, i);
        GmCutout *cutout;

	if (JSON_NODE_HOLDS_OBJECT (element_node)) {
	  cutout = GM_CUTOUT (json_gobject_deserialize (GM_TYPE_CUTOUT, element_node));
          g_list_store_append (cutouts, cutout);
	} else {
	  return FALSE;
	}
      }
      g_value_set_object (value, cutouts);
      return TRUE;
    }
    return FALSE;
  } else {
    return json_serializable_default_deserialize_property (serializable,
							   property_name,
							   value,
							   pspec,
							   property_node);
  }
  return FALSE;
}


static void
gm_display_panel_json_serializable_iface_init (JsonSerializableIface *iface)
{
  iface->serialize_property = gm_display_panel_serializable_serialize_property;
  iface->deserialize_property = gm_display_panel_serializable_deserialize_property;
}


static void
gm_display_panel_finalize (GObject *object)
{
  GmDisplayPanel *self = GM_DISPLAY_PANEL(object);

  g_clear_object (&self->cutouts);
  g_clear_pointer (&self->name, g_free);

  G_OBJECT_CLASS (gm_display_panel_parent_class)->finalize (object);
}


static void
gm_display_panel_class_init (GmDisplayPanelClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = gm_display_panel_get_property;
  object_class->set_property = gm_display_panel_set_property;
  object_class->finalize = gm_display_panel_finalize;

  /**
   * GmDisplayPanel:name:
   *
   * The name of the display
   *
   * Since: 0.0.1
   */
  props[PROP_NAME] =
    g_param_spec_string ("name", "", "",
                         NULL,
                         G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);

  /**
   * GmDisplayPanel:cutouts:
   *
   * The display cutouts as `GListModel` of [class@Cutout].
   *
   * Since: 0.0.1
   */
  props[PROP_CUTOUTS] =
    g_param_spec_object ("cutouts", "", "",
                         G_TYPE_LIST_STORE,
                         G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * GmDisplayPanel:x-res:
   *
   * The panel resolution in pixels in the x direction
   *
   * Since: 0.0.1
   */
  props[PROP_X_RES] =
    g_param_spec_int ("x-res", "", "",
                      0, G_MAXINT, 0,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * GmDisplayPanel:y-res:
   *
   * The panel resolution in pixels in the y direction
   *
   * Since: 0.0.1
   */
  props[PROP_Y_RES] =
    g_param_spec_int ("y-res", "", "",
                      0, G_MAXINT, 0,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * GmDisplayPanel:border-radius:
   *
   * The border radius of the panel edges in device pixels
   * If a single border radius isn't enough use multiple [type@Cutout].
   *
   * Since: 0.0.1
   */
  props[PROP_BORDER_RADIUS] =
    g_param_spec_int ("border-radius", "", "",
                      0, G_MAXINT, 0,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * GmDisplayPanel:width:
   *
   * The display width in millimeters
   *
   * Since: 0.0.1
   */
  props[PROP_WIDTH] =
    g_param_spec_int ("width", "", "",
                      0, G_MAXINT, 0,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * GmDisplayPanel:height:
   *
   * The display height in millimeters
   *
   * Since: 0.0.1
   */
  props[PROP_HEIGHT] =
    g_param_spec_int ("height", "", "",
                      0, G_MAXINT, 0,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
}


static void
gm_display_panel_init (GmDisplayPanel *self)
{
  self->cutouts = g_list_store_new (GM_TYPE_CUTOUT);
}

/**
 * gm_display_panel_new:
 *
 * Constructs a new display panel object.
 *
 * Returns: The new display panel object
 *
 * Since: 0.0.1
 */
GmDisplayPanel *
gm_display_panel_new (void)
{
  return GM_DISPLAY_PANEL (g_object_new (GM_TYPE_DISPLAY_PANEL, NULL));
}

/**
 * gm_display_panel_new_from_data:
 * @data: The panel's data as JSON
 * @error: Return location for an error
 *
 * Constructs a new display panel based on the given data. If that fails
 * `NULL` is returned and `error` describes the error that occurred.
 *
 * Returns: The new display panel object
 *
 * Since: 0.0.1
 */
GmDisplayPanel *
gm_display_panel_new_from_data (const gchar *data, GError **error)
{
  g_autoptr (JsonNode) node = json_from_string(data, error);
  if (!node)
    return NULL;

  return GM_DISPLAY_PANEL (json_gobject_deserialize (GM_TYPE_DISPLAY_PANEL, node));
}

/**
 * gm_display_panel_new_from_resource:
 * @resource_name: A path to a gresource
 * @error: Return location for an error
  *
 * Constructs a new display panel by fetching the data from the given
 * GResource. If that fails `NULL` is returned and `error` describes
 * the error that occurred.
 *
 * Returns: The new display panel object
 *
 * Since: 0.0.1
 */
GmDisplayPanel *
gm_display_panel_new_from_resource (const gchar *resource_name, GError **error)
{
  g_autoptr (GBytes) bytes = NULL;

  g_return_val_if_fail (resource_name && resource_name[0], NULL);

  /* Make sure resources are initialized */
  gm_init ();

  bytes = g_resources_lookup_data (resource_name, 0, error);
  if (bytes == NULL)
    return NULL;

  return GM_DISPLAY_PANEL (gm_display_panel_new_from_data ((const char *)g_bytes_get_data (bytes, NULL),
                                                           error));
}

/**
 * gm_display_panel_get_name:
 *
 * Gets the panel's name.
 *
 * Returns: The panel's name
 *
 * Since: 0.0.1
 */
const char *
gm_display_panel_get_name (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), NULL);

  return self->name;
}

/**
 * gm_display_panel_get_cutouts:
 * @self: The display panel
 *
 * Get the display cutouts.
 *
 * Returns: (transfer none): The display cutouts
 *
 * Since: 0.0.1
 */
GListModel *
gm_display_panel_get_cutouts (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), NULL);

  return G_LIST_MODEL (self->cutouts);
}

/**
 * gm_display_panel_get_x_res:
 * @self: The display panel
 *
 * Gets the panels resolution (in pixels) in the x direction
 *
 * Returns: The x resolution.
 *
 * Since: 0.0.1
 */
int
gm_display_panel_get_x_res (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), 0);

  return self->x_res;
}

/**
 * gm_display_panel_get_y_res:
 * @self: The display panel
 *
 * Gets the panels resolution (in pixels) in the y direction.
 *
 * Returns: The y resolution.
 *
 * Since: 0.0.1
 */
int
gm_display_panel_get_y_res (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), 0);

  return self->y_res;
}

/**
 * gm_display_panel_get_border_radius:
 * @self: The display panel
 *
 * Gets the panels border radius. 0 indicates rectangular corners.  If
 * given applies to all corners of the panel.
 *
 * Returns: The panel's border radius.
 *
 * Since: 0.0.1
 */
int
gm_display_panel_get_border_radius (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), 0);

  return self->border_radius;
}

/**
 * gm_display_panel_get_width:
 * @self: The display panel
 *
 * Gets the panels width in mm.
 *
 * Returns: The panel's width.
 *
 * Since: 0.0.1
 */
int
gm_display_panel_get_width (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), 0);

  return self->width;
}

/**
 * gm_display_panel_get_height:
 * @self: The display panel
 *
 * Gets the panels height in mm.
 *
 * Returns: The panel's height.
 *
 * Since: 0.0.1
 */
int
gm_display_panel_get_height (GmDisplayPanel *self)
{
  g_return_val_if_fail (GM_IS_DISPLAY_PANEL (self), 0);

  return self->height;
}
