/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */
#pragma once

#include <glib-object.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define CUI_TYPE_CALL (cui_call_get_type ())
G_DECLARE_INTERFACE (CuiCall, cui_call, CUI, CALL, GObject)

/**
 * CuiCallState:
 * @CUI_CALL_STATE_UNKNOWN: Call state unknown
 * @CUI_CALL_STATE_ACTIVE: Call is active
 * @CUI_CALL_STATE_HELD: Call is held
 * @CUI_CALL_STATE_CALLING: Call is being placed
 * @CUI_CALL_STATE_INCOMING: Call is incoming
 * @CUI_CALL_STATE_DISCONNECTED: Call has ended
 *
 * The call state of a [iface@Cui.Call]
 */
typedef enum
{
  CUI_CALL_STATE_UNKNOWN = 0,
  CUI_CALL_STATE_ACTIVE = 1,
  CUI_CALL_STATE_HELD = 2,
  CUI_CALL_STATE_CALLING = 3,
  CUI_CALL_STATE_INCOMING = 5,
  CUI_CALL_STATE_DISCONNECTED = 7
} CuiCallState;

/**
 * CuiCallInterface:
 * @parent_iface: The parent interface
 * @get_avatar_icon: Get current calls's avatar icon
 * @get_display_name: Get current calls's display name
 * @get_id: Get current calls's id
 * @get_state: Get the call's state
 * @get_encrypted: Gets whether the call is encrypted
 * @get_can_dtmf: Gets whether the call can handle DTMF
 * @accept: Accept the incoming call
 * @hang_up: Hang-up an ongoing call or reject an incoming call
 * @send_dtmf: Send DTMF
 */
struct _CuiCallInterface {
  GTypeInterface parent_iface;

  GLoadableIcon *(*get_avatar_icon)        (CuiCall *self);
  const char    *(*get_display_name)       (CuiCall *self);
  const char    *(*get_id)                 (CuiCall *self);
  CuiCallState   (*get_state)              (CuiCall *self);
  gboolean       (*get_encrypted)          (CuiCall *self);
  gboolean       (*get_can_dtmf)           (CuiCall *self);
  gdouble        (*get_active_time)        (CuiCall *self);

  void           (*accept)                 (CuiCall *self);
  void           (*hang_up)                (CuiCall *self);
  void           (*send_dtmf)              (CuiCall *self, const gchar *dtmf);
};

GLoadableIcon *cui_call_get_avatar_icon (CuiCall *self);
const char  *cui_call_get_display_name (CuiCall *self);
const char  *cui_call_get_id           (CuiCall *self);
CuiCallState cui_call_get_state        (CuiCall *self);
gboolean     cui_call_get_encrypted    (CuiCall *self);
gboolean     cui_call_get_can_dtmf     (CuiCall *self);
gdouble      cui_call_get_active_time  (CuiCall *self);

void         cui_call_accept           (CuiCall *self);
void         cui_call_hang_up          (CuiCall *self);
void         cui_call_send_dtmf (CuiCall *self, const gchar *dtmf);

const char  *cui_call_state_to_string  (CuiCallState state);
char        *cui_call_format_duration  (double duration);
