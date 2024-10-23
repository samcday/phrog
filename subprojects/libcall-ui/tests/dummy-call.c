/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Evangelos Ribeiro Tzaras <devrtz@fortysixandtwo.eu>
 *
 * heavily based on examples/cui-demo-call.c by Guido Günther <agx@sigxcpu.org>
 */

#include "dummy-call.h"

#include <glib/gi18n.h>

enum {
  PROP_0,
  PROP_AVATAR_ICON,
  PROP_DISPLAY_NAME,
  PROP_ID,
  PROP_STATE,
  PROP_ENCRYPTED,
  PROP_CAN_DTMF,
  PROP_ACTIVE_TIME,
  PROP_LAST_PROP,
};
static GParamSpec *props[PROP_LAST_PROP];

struct _CuiDummyCall
{
  GObject       parent_instance;

  char         *id;
  char         *display_name;
  CuiCallState  state;
  gboolean      encrypted;
  gboolean      can_dtmf;
};

static void cui_dummy_cui_call_interface_init (CuiCallInterface *iface);
G_DEFINE_TYPE_WITH_CODE (CuiDummyCall, cui_dummy_call, G_TYPE_OBJECT,
                         G_IMPLEMENT_INTERFACE (CUI_TYPE_CALL,
                                                cui_dummy_cui_call_interface_init))


static void
cui_dummy_call_get_property (GObject    *object,
                             guint       prop_id,
                             GValue     *value,
                             GParamSpec *pspec)
{
  CuiDummyCall *self = CUI_DUMMY_CALL (object);

  switch (prop_id) {
  case PROP_ID:
    g_value_set_string (value, self->id);
    break;
  case PROP_AVATAR_ICON:
    g_value_set_object (value, NULL);
    break;
  case PROP_DISPLAY_NAME:
    g_value_set_string (value, self->display_name);
    break;
  case PROP_STATE:
    g_value_set_enum (value, self->state);
    break;
  case PROP_ENCRYPTED:
    g_value_set_boolean (value, self->encrypted);
    break;
  case PROP_CAN_DTMF:
    g_value_set_boolean (value, self->can_dtmf);
    break;
  case PROP_ACTIVE_TIME:
    g_value_set_double (value, 0.0);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, prop_id, pspec);
  }
}


static void
cui_dummy_call_finalize (GObject *object)
{
  CuiDummyCall *self = CUI_DUMMY_CALL (object);

  g_free (self->id);
  g_free (self->display_name);

  G_OBJECT_CLASS (cui_dummy_call_parent_class)->finalize (object);
}


static void
cui_dummy_call_class_init (CuiDummyCallClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->get_property = cui_dummy_call_get_property;
  object_class->finalize = cui_dummy_call_finalize;

  g_object_class_override_property (object_class,
                                    PROP_ID,
                                    "id");
  props[PROP_ID] = g_object_class_find_property (object_class, "id");

  g_object_class_override_property (object_class,
                                    PROP_AVATAR_ICON,
                                    "avatar-icon");
  props[PROP_AVATAR_ICON] = g_object_class_find_property (object_class, "avatar-icon");

  g_object_class_override_property (object_class,
                                    PROP_DISPLAY_NAME,
                                    "display-name");
  props[PROP_DISPLAY_NAME] = g_object_class_find_property (object_class, "display-name");

  g_object_class_override_property (object_class,
                                    PROP_STATE,
                                    "state");
  props[PROP_STATE] = g_object_class_find_property (object_class, "state");

  g_object_class_override_property (object_class,
                                    PROP_ENCRYPTED,
                                    "encrypted");
  props[PROP_ENCRYPTED] = g_object_class_find_property (object_class, "encrypted");

  g_object_class_override_property (object_class,
                                    PROP_CAN_DTMF,
                                    "can-dtmf");
  props[PROP_CAN_DTMF] = g_object_class_find_property (object_class, "can-dtmf");

  g_object_class_override_property (object_class,
                                    PROP_ACTIVE_TIME,
                                    "active-time");
  props[PROP_ACTIVE_TIME] = g_object_class_find_property (object_class, "active-time");

}


static const char *
cui_dummy_call_get_id (CuiCall *call)
{
  g_return_val_if_fail (CUI_IS_DUMMY_CALL (call), NULL);

  return CUI_DUMMY_CALL (call)->id;
}


static const char *
cui_dummy_call_get_display_name (CuiCall *call)
{
  g_return_val_if_fail (CUI_IS_DUMMY_CALL (call), NULL);

  return CUI_DUMMY_CALL (call)->display_name;
}


static CuiCallState
cui_dummy_call_get_state (CuiCall *call)
{
  g_return_val_if_fail (CUI_IS_DUMMY_CALL (call), CUI_CALL_STATE_UNKNOWN);

  return CUI_DUMMY_CALL (call)->state;
}


static gboolean
cui_dummy_call_get_encrypted (CuiCall *call)
{
  g_return_val_if_fail (CUI_IS_DUMMY_CALL (call), CUI_CALL_STATE_UNKNOWN);

  return CUI_DUMMY_CALL (call)->encrypted;
}


static gboolean
cui_dummy_call_get_can_dtmf (CuiCall *call)
{
  g_return_val_if_fail (CUI_IS_DUMMY_CALL (call), FALSE);

  return CUI_DUMMY_CALL (call)->can_dtmf;
}


static void
cui_dummy_call_accept (CuiCall *call)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (call));

  CUI_DUMMY_CALL (call)->state = CUI_CALL_STATE_ACTIVE;
  g_object_notify_by_pspec (G_OBJECT (call), props[PROP_STATE]);
}


static void
cui_dummy_call_hang_up (CuiCall *call)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (call));

  CUI_DUMMY_CALL (call)->state = CUI_CALL_STATE_DISCONNECTED;
  g_object_notify_by_pspec (G_OBJECT (call), props[PROP_STATE]);
}


static void
cui_dummy_call_send_dtmf (CuiCall *call, const gchar *dtmf)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (call));

  g_message ("DTMF: %s", dtmf);
}


static void
cui_dummy_cui_call_interface_init (CuiCallInterface *iface)
{
  iface->get_id = cui_dummy_call_get_id;
  iface->get_display_name = cui_dummy_call_get_display_name;
  iface->get_state = cui_dummy_call_get_state;
  iface->get_encrypted = cui_dummy_call_get_encrypted;
  iface->get_can_dtmf = cui_dummy_call_get_can_dtmf;

  iface->accept = cui_dummy_call_accept;
  iface->hang_up = cui_dummy_call_hang_up;
  iface->send_dtmf = cui_dummy_call_send_dtmf;
}


static void
cui_dummy_call_init (CuiDummyCall *self)
{
  self->display_name = g_strdup ("John Doe");
  self->id = g_strdup ("0800 1234");
  self->state = CUI_CALL_STATE_INCOMING;
  self->can_dtmf = TRUE;
}


CuiDummyCall *
cui_dummy_call_new (void)
{
   return g_object_new (CUI_TYPE_DUMMY_CALL, NULL);
}


void
cui_dummy_call_set_id (CuiDummyCall *self,
                       const char   *id)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (self));

  g_free (self->id);
  self->id = g_strdup (id);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_ID]);
}


void
cui_dummy_call_set_display_name (CuiDummyCall *self,
                                 const char   *display_name)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (self));

  g_free (self->display_name);
  self->display_name = g_strdup (display_name);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_DISPLAY_NAME]);
}


void
cui_dummy_call_set_state (CuiDummyCall *self,
                          CuiCallState  state)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (self));

  if (self->state == state)
    return;

  self->state = state;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_STATE]);
}


void
cui_dummy_call_set_can_dtmf (CuiDummyCall *self,
                             gboolean      enabled)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (self));

  if (self->can_dtmf == enabled)
    return;

  self->can_dtmf = enabled;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_CAN_DTMF]);
}


void
cui_dummy_call_set_encrypted (CuiDummyCall *self,
                              gboolean      enabled)
{
  g_return_if_fail (CUI_IS_DUMMY_CALL (self));

  if (self->encrypted == enabled)
    return;

  self->encrypted = enabled;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_CAN_DTMF]);
}
