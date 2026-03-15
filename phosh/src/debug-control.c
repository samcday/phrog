/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define G_LOG_DOMAIN "phosh-debug-control"

#include "phosh-config.h"

#include "debug-control.h"
#include "phosh-enums.h"
#include "shell-priv.h"

#include <gio/gio.h>

#define DEBUG_CONTROL_DBUS_PATH "/mobi/phosh/Shell/DebugControl"
#define DEBUG_CONTROL_DBUS_NAME "mobi.phosh.Shell.DebugControl"

/**
 * PhoshDebugControl:
 *
 * DBus Debug control interface
 */

enum {
  PROP_0,
  PROP_EXPORTED,
  PROP_LAST_PROP
};
static GParamSpec *props[PROP_LAST_PROP];

struct _PhoshDebugControl {
  PhoshDBusDebugControlSkeleton parent;

  guint dbus_name_id;
  gboolean exported;
};

static void phosh_dbus_debug_control_iface_init (PhoshDBusDebugControlIface *iface);

G_DEFINE_TYPE_WITH_CODE (PhoshDebugControl, phosh_debug_control,
                         PHOSH_DBUS_TYPE_DEBUG_CONTROL_SKELETON,
                         G_IMPLEMENT_INTERFACE (PHOSH_DBUS_TYPE_DEBUG_CONTROL,
                                                phosh_dbus_debug_control_iface_init))

static void
phosh_dbus_debug_control_iface_init (PhoshDBusDebugControlIface *iface)
{
}


static void
on_bus_acquired (GDBusConnection *connection, const char *name, gpointer user_data)
{
  PhoshDebugControl *self = user_data;
  g_autoptr (GError) err = NULL;

  if (g_dbus_interface_skeleton_export (G_DBUS_INTERFACE_SKELETON (self),
                                        connection,
                                        DEBUG_CONTROL_DBUS_PATH,
                                        &err)) {
    self->exported = TRUE;
    g_object_notify_by_pspec (G_OBJECT (self), props[PROP_EXPORTED]);
    g_debug ("Debug interface exported on '%s'", DEBUG_CONTROL_DBUS_NAME);
  } else {
    g_warning ("Failed to export on %s: %s", DEBUG_CONTROL_DBUS_NAME, err->message);
  }
}


static void
phosh_debug_control_set_property (GObject      *object,
                                  guint         property_id,
                                  const GValue *value,
                                  GParamSpec   *pspec)
{
  PhoshDebugControl *self = PHOSH_DEBUG_CONTROL (object);

  switch (property_id) {
  case PROP_EXPORTED:
    phosh_debug_control_set_exported (self, g_value_get_boolean (value));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_debug_control_get_property (GObject    *object,
                                  guint       property_id,
                                  GValue     *value,
                                  GParamSpec *pspec)
{
  PhoshDebugControl *self = PHOSH_DEBUG_CONTROL (object);

  switch (property_id) {
  case PROP_EXPORTED:
    g_value_set_boolean (value, self->exported);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_debug_control_dispose (GObject *object)
{
  PhoshDebugControl *self = PHOSH_DEBUG_CONTROL (object);

  if (self->exported) {
    g_dbus_interface_skeleton_unexport (G_DBUS_INTERFACE_SKELETON (self));
    self->exported = FALSE;
  }
  g_clear_handle_id (&self->dbus_name_id, g_bus_unown_name);

  G_OBJECT_CLASS (phosh_debug_control_parent_class)->dispose (object);
}


static void
phosh_debug_control_class_init (PhoshDebugControlClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = phosh_debug_control_get_property;
  object_class->set_property = phosh_debug_control_set_property;
  object_class->dispose = phosh_debug_control_dispose;

  props[PROP_EXPORTED] =
    g_param_spec_boolean ("exported", "", "",
                          FALSE,
                          G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY | G_PARAM_STATIC_STRINGS);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
}


static void
phosh_debug_control_init (PhoshDebugControl *self)
{
  g_object_bind_property (phosh_shell_get_default (),
                          "log-domains",
                          self,
                          "log-domains",
                          G_BINDING_SYNC_CREATE | G_BINDING_BIDIRECTIONAL);
}


PhoshDebugControl *
phosh_debug_control_new (void)
{
  return g_object_new (PHOSH_TYPE_DEBUG_CONTROL, NULL);
}


void
phosh_debug_control_set_exported (PhoshDebugControl *self, gboolean exported)
{
  g_assert (PHOSH_IS_DEBUG_CONTROL (self));

  if (self->exported == exported)
    return;

  if (exported) {
    self->dbus_name_id = g_bus_own_name (G_BUS_TYPE_SESSION,
                                         DEBUG_CONTROL_DBUS_NAME,
                                         G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT |
                                         G_BUS_NAME_OWNER_FLAGS_REPLACE,
                                         on_bus_acquired,
                                         NULL,
                                         NULL,
                                         self,
                                         NULL);
  } else {
    g_clear_handle_id (&self->dbus_name_id, g_bus_unown_name);
    self->exported = FALSE;
    g_object_notify_by_pspec (G_OBJECT (self), props[PROP_EXPORTED]);
  }
}
