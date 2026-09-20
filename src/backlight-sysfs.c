/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 *
 * Somewhat based on gsd-backlight which is
 * Copyright (C) 2018 Red Hat Inc.
 */

#define G_LOG_DOMAIN "phosh-backlight-sysfs"

#include "phosh-config.h"

#include "backlight-sysfs.h"
#include "dbus/login1-session-dbus.h"
#include "udev-manager.h"

#include <gudev/gudev.h>

/**
 * PhoshBacklightSysfs:
 *
 * A backlight managed via sysfs / logind
 */

enum {
  PROP_0,
  PROP_CONNECTOR_NAME,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];

struct _PhoshBacklightSysfs {
  PhoshBacklight parent;

  char          *connector_name;

  char          *device_name;
  char          *device_path;
  char          *brightness_path;

  GUdevDevice   *device;
  PhoshDBusLoginSession *session_proxy;
};

static void initable_iface_init (GInitableIface *iface);

G_DEFINE_TYPE_WITH_CODE (PhoshBacklightSysfs, phosh_backlight_sysfs, PHOSH_TYPE_BACKLIGHT,
                         G_IMPLEMENT_INTERFACE (G_TYPE_INITABLE, initable_iface_init))

static void
on_dbus_login_session_brightness_set (GObject      *source_object,
                                      GAsyncResult *res,
                                      gpointer      user_data)
{
  PhoshDBusLoginSession *session_proxy = PHOSH_DBUS_LOGIN_SESSION (source_object);
  g_autoptr (GTask) task = G_TASK (user_data);
  int brightness;
  g_autoptr (GError) error = NULL;

  brightness = GPOINTER_TO_INT (g_task_get_task_data (task));
  if (!phosh_dbus_login_session_call_set_brightness_finish (session_proxy,
                                                            res,
                                                            &error)) {
    g_task_return_error (task, g_steal_pointer (&error));
    return;
  }

  g_task_return_int (task, brightness);
}


static int
phosh_backlight_sysfs_set_level_finish (PhoshBacklight  *backlight,
                                        GAsyncResult    *result,
                                        GError         **error)
{
  PhoshBacklightSysfs *self = PHOSH_BACKLIGHT_SYSFS (backlight);

  g_return_val_if_fail (g_task_is_valid (result, self), -1);

  return g_task_propagate_int (G_TASK (result), error);
}


static void
phosh_backlight_sysfs_set_level (PhoshBacklight      *backlight,
                                 int                  brightness,
                                 GCancellable        *cancellable,
                                 GAsyncReadyCallback  callback,
                                 gpointer             user_data)
{
  PhoshBacklightSysfs *self = PHOSH_BACKLIGHT_SYSFS (backlight);
  g_autoptr (GTask) task = NULL;

  g_return_if_fail (PHOSH_IS_BACKLIGHT_SYSFS (self));

  task = g_task_new (self, cancellable, callback, user_data);
  g_task_set_task_data (task, GINT_TO_POINTER (brightness), NULL);
  g_task_set_source_tag (task, phosh_backlight_sysfs_set_level);

  if (!self->session_proxy)
    return;

  g_debug ("Setting brightness via logind: %d", brightness);
  phosh_dbus_login_session_call_set_brightness (self->session_proxy,
                                                "backlight",
                                                self->device_name,
                                                brightness,
                                                cancellable,
                                                on_dbus_login_session_brightness_set,
                                                g_steal_pointer (&task));
}


static void
phosh_backlight_sysfs_update (PhoshBacklightSysfs *self)
{
  g_autoptr (GError) err = NULL;
  g_autofree char *contents = NULL;
  int level;

  if (!g_file_get_contents (self->brightness_path, &contents, NULL, &err)) {
    g_warning ("Backlight %s: Could not get brightness from sysfs: %s",
               self->connector_name,
               err->message);
    return;
  }

  level = g_ascii_strtoll (contents, NULL, 0);
  phosh_backlight_backend_update_level (PHOSH_BACKLIGHT (self), level);
}


static void
on_backlight_changed (PhoshBacklightSysfs *self,
                      GUdevDevice         *udev_device,
                      PhoshUdevManager    *udev_manager)
{
  g_assert (PHOSH_IS_BACKLIGHT_SYSFS (self));
  g_assert (PHOSH_IS_UDEV_MANAGER (udev_manager));

  if (g_strcmp0 (g_udev_device_get_sysfs_path (udev_device), self->device_path) != 0)
    return;

  phosh_backlight_sysfs_update (self);
}


static gboolean
phosh_backlight_sysfs_get_udev_info (GUdevDevice         *device,
                                     int                 *minout,
                                     int                 *maxout,
                                     PhoshBacklightScale *scaleout,
                                     GError             **err)
{
  int min, max;
  const char *device_type, *scale_str;
  PhoshBacklightScale scale = PHOSH_BACKLIGHT_SCALE_UNKNOWN;

  max = g_udev_device_get_sysfs_attr_as_int (device, "max_brightness");
  min = MAX (1, max / 100);

  /* If the interface has less than 100 possible values, and it is of type
   * raw, then assume that 0 does not turn off the backlight completely. */
  device_type = g_udev_device_get_sysfs_attr (device, "type");
  if (max < 99 && g_strcmp0 (device_type, "raw") == 0)
    min = 0;

  /* Ignore a backlight which has no steps. */
  if (min >= max) {
    g_set_error (err, G_IO_ERROR, G_IO_ERROR_FAILED,
                 "Backlight has an invalid maximum brightness [%d,%d]", min, max);
    return FALSE;
  }

  scale_str = g_udev_device_get_sysfs_attr (device, "scale");
  if (scale_str) {
    if (g_str_equal (scale_str, "unknown")) {
      scale = PHOSH_BACKLIGHT_SCALE_UNKNOWN;
    } else if (g_str_equal (scale_str, "linear")) {
      scale = PHOSH_BACKLIGHT_SCALE_LINEAR;
    } else if (g_str_equal (scale_str, "non-linear")) {
      scale = PHOSH_BACKLIGHT_SCALE_NON_LINEAR;
    } else {
      g_warning ("Unknown brightness scale '%s'", scale_str);
    }
  }

  if (minout)
    *minout = min;
  if (maxout)
    *maxout = max;
  if (scaleout)
    *scaleout = scale;

  return TRUE;
}


static gboolean
initable_init (GInitable *initable, GCancellable *cancel, GError **error)
{
  PhoshBacklightSysfs *self = PHOSH_BACKLIGHT_SYSFS (initable);
  PhoshUdevManager *udev_manager = phosh_udev_manager_get_default ();
  PhoshBacklightScale scale;
  int min = 0, max = 0;

  if (!self->connector_name) {
    g_set_error (error, G_IO_ERROR, G_IO_ERROR_FAILED,
                 "No connector name given");
    return FALSE;
  }

  self->device = phosh_udev_manager_find_backlight (udev_manager, self->connector_name);
  if (!self->device) {
    g_set_error (error, G_IO_ERROR, G_IO_ERROR_NOT_SUPPORTED,
                 "No matching backlight device found");
    return FALSE;
  }

  if (!phosh_backlight_sysfs_get_udev_info (self->device, &min, &max, &scale, error))
    return FALSE;

  phosh_backlight_set_range (PHOSH_BACKLIGHT (self), min, max, scale);

  self->device_name = g_strdup (g_udev_device_get_name (self->device));
  self->device_path = realpath (g_udev_device_get_sysfs_path (self->device), NULL);
  if (!self->device_path) {
    g_set_error (error, G_IO_ERROR,
                 g_io_error_from_errno (errno),
                 "Could not get real path for %s", self->device_name);
    return FALSE;
  }
  self->brightness_path = g_build_filename (self->device_path, "brightness", NULL);
  self->session_proxy = phosh_udev_manager_get_session_proxy (udev_manager);

  g_signal_connect_object (udev_manager, "backlight-changed",
                           G_CALLBACK (on_backlight_changed),
                           self,
                           G_CONNECT_SWAPPED);
  phosh_backlight_sysfs_update (self);

  return TRUE;
}


static void
initable_iface_init (GInitableIface *iface)
{
  iface->init = initable_init;
}


static void
phosh_backlight_sysfs_set_property (GObject      *object,
                                    guint         property_id,
                                    const GValue *value,
                                    GParamSpec   *pspec)
{
  PhoshBacklightSysfs *self = PHOSH_BACKLIGHT_SYSFS (object);

  switch (property_id) {
  case PROP_CONNECTOR_NAME:
    self->connector_name = g_value_dup_string (value);
    phosh_backlight_set_name (PHOSH_BACKLIGHT (self), self->connector_name);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_backlight_sysfs_dispose (GObject *object)
{
  PhoshBacklightSysfs *self = PHOSH_BACKLIGHT_SYSFS (object);

  g_clear_pointer (&self->brightness_path, g_free);
  g_clear_pointer (&self->device_path, g_free);
  g_clear_pointer (&self->device_name, g_free);
  g_clear_pointer (&self->connector_name, g_free);
  g_clear_object (&self->device);
  g_clear_object (&self->session_proxy);

  G_OBJECT_CLASS (phosh_backlight_sysfs_parent_class)->dispose (object);
}


static void
phosh_backlight_sysfs_class_init (PhoshBacklightSysfsClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  PhoshBacklightClass *backlight_class = PHOSH_BACKLIGHT_CLASS (klass);

  object_class->set_property = phosh_backlight_sysfs_set_property;
  object_class->dispose = phosh_backlight_sysfs_dispose;

  backlight_class->set_level = phosh_backlight_sysfs_set_level;
  backlight_class->set_level_finish = phosh_backlight_sysfs_set_level_finish;

  props[PROP_CONNECTOR_NAME] =
    g_param_spec_string ("connector-name", "", "",
                         NULL,
                         G_PARAM_WRITABLE | G_PARAM_CONSTRUCT_ONLY | G_PARAM_STATIC_STRINGS);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
}


static void
phosh_backlight_sysfs_init (PhoshBacklightSysfs *self)
{
}


PhoshBacklightSysfs *
phosh_backlight_sysfs_new (const char *connector_name, GError **error)
{
  return g_initable_new (PHOSH_TYPE_BACKLIGHT_SYSFS, NULL, error,
                         "connector-name", connector_name,
                         NULL);
}
