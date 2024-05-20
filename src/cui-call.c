/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "cui-config.h"

#include "cui-call.h"
#include "cui-enums.h"

#include <gio/gio.h>
#include <glib/gi18n-lib.h>

G_DEFINE_INTERFACE (CuiCall, cui_call, G_TYPE_OBJECT)


/**
 * CuiCall:
 *
 * An interface for phone calls.
 *
 * Objects implementing the `CuiCall` interface can be handled in a
 * [class@Cui.CallDisplay]. [class@Cui.CallDisplay] will invoke
 * this interface's implementation to accept, hang-up calls. etc.
 */

void
cui_call_default_init (CuiCallInterface *iface)
{
  /**
   * CuiCall:display-name:
   *
   * The display name. E.g. the name of the caller instead of the plain
   * phone number.
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_string ("display-name",
                         "",
                         "",
                         NULL,
                         G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));
  /**
   * CuiCall:avatar-icon:
   *
   * The call's avatar icon
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_object ("avatar-icon",
                         "",
                         "",
                         G_TYPE_LOADABLE_ICON,
                         G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));
  /**
   * CuiCall:id:
   *
   * The call's id, e.g. a phone number.
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_string ("id",
                         "",
                         "",
                         NULL,
                         G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));

  /**
   * CuiCall:state:
   *
   * The call's state.
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_enum ("state",
                       "",
                       "",
                       CUI_TYPE_CALL_STATE,
                       0,
                       G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));

  /**
   * CuiCall:encrypted:
   *
   * Whether the call is encrypted
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_boolean ("encrypted",
                          "",
                          "",
                          FALSE,
                          G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));

  /**
   * CuiCall:can-dtmf
   *
   * Whether the call can have DTMF
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_boolean ("can-dtmf",
                          "",
                          "",
                          FALSE,
                          G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));

  /**
   * CuiCall:active-time
   *
   * The time in seconds that this call has been active.
   * This corresponds to the time displayed in the call display and
   * it is the responsibility of the implementing class to update
   * this property about once a second.
   */
  g_object_interface_install_property (
    iface,
    g_param_spec_double ("active-time",
                         "",
                         "",
                         0.0,
                         G_MAXDOUBLE,
                         0.0,
                         G_PARAM_READABLE | G_PARAM_EXPLICIT_NOTIFY));
}


/**
 * cui_call_get_avatar_icon:
 * @self: The call
 *
 * Get the avatar icon.
 *
 * Returns: (transfer none)(nullable): The icon as `GLoadableIcon`
 */
GLoadableIcon *
cui_call_get_avatar_icon (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), NULL);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_avatar_icon, NULL);

  return iface->get_avatar_icon (self);
}


const char *
cui_call_get_display_name (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), NULL);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_display_name, NULL);

  return iface->get_display_name (self);
}


const char *
cui_call_get_id (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), NULL);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_id, NULL);

  return iface->get_id (self);
}


CuiCallState
cui_call_get_state (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), CUI_CALL_STATE_UNKNOWN);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_state, CUI_CALL_STATE_UNKNOWN);

  return iface->get_state (self);
}


gboolean
cui_call_get_encrypted (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), FALSE);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_encrypted, FALSE);

  return iface->get_encrypted (self);
}


gdouble
cui_call_get_active_time (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), FALSE);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_active_time, 0.0);

  return iface->get_active_time (self);
}


gboolean
cui_call_get_can_dtmf (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_val_if_fail (CUI_IS_CALL (self), FALSE);

  iface = CUI_CALL_GET_IFACE (self);
  g_return_val_if_fail (iface->get_can_dtmf, FALSE);

  return iface->get_can_dtmf (self);
}

/**
 * cui_call_accept:
 * @self: The call
 *
 * Accept the call.
 */
void
cui_call_accept (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_if_fail (CUI_IS_CALL (self));

  iface = CUI_CALL_GET_IFACE (self);
  g_return_if_fail (iface->accept);

  iface->accept (self);
}

/**
 * cui_call_hang_up:
 * @self: The call
 *
 * Hang up the call.
 */
void
cui_call_hang_up (CuiCall *self)
{
  CuiCallInterface *iface;

  g_return_if_fail (CUI_IS_CALL (self));

  iface = CUI_CALL_GET_IFACE (self);
  g_return_if_fail (iface->hang_up);

  iface->hang_up (self);
}

/**
 * cui_call_send_dtmf:
 * @self: The call
 * @dtmf: The DTMF data
 *
 * Send DTMF to the call.
 */
void
cui_call_send_dtmf (CuiCall *self, const gchar *dtmf)
{
  CuiCallInterface *iface;

  g_return_if_fail (CUI_IS_CALL (self));

  if (!cui_call_get_can_dtmf (self))
    return;

  iface = CUI_CALL_GET_IFACE (self);
  g_return_if_fail (iface->send_dtmf);

  iface->send_dtmf (self, dtmf);
}

/**
 * cui_call_state_to_string:
 * @state: The #CuiCallState
 *
 * Returns: (transfer none): A human readable state description
 */
const char *
cui_call_state_to_string (CuiCallState state)
{
  switch (state) {
  case CUI_CALL_STATE_ACTIVE:
    return _("Call active");
  case CUI_CALL_STATE_HELD:
    return _("Call held");
  case CUI_CALL_STATE_CALLING:
    return _("Calling…");
  case CUI_CALL_STATE_INCOMING:
    return _("Incoming call");
  case CUI_CALL_STATE_DISCONNECTED:
    return _("Call ended");
  case CUI_CALL_STATE_UNKNOWN:
  default:
    return _("Unknown");
  }
}

/**
 * cui_call_format_duration:
 * @duration: The call duration
 *
 * Formats the call duration as mm:ss or hh:mm:ss depending
 * on the duration.
 *
 * Returns: (transfer full): The call duration as text
 */
char *
cui_call_format_duration (double duration)
{
#define MINUTE 60
#define HOUR   (60 * MINUTE)
  guint seconds, minutes;
  GString *str = g_string_new ("");

  if (duration > HOUR) {
    int hours = (int) (duration / HOUR);
    g_string_append_printf (str, "%u:", hours);
    duration -= (hours * HOUR);
  }

  minutes = (int) (duration / MINUTE);
  seconds = duration - (minutes * MINUTE);
  g_string_append_printf (str, "%02u:%02u", minutes, seconds);

  return g_string_free (str, FALSE);
#undef HOUR
#undef MINUTE
}
