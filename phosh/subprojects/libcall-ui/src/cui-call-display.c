/*
 * Copyright (C) 2021, 2022 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 *         Evangelos Ribeiro Tzaras <devrtz@fortysixandtwo.eu>
 *
 * Somewhat based on call's call-display by:
 * Author: Bob Ham <bob.ham@puri.sm>
 */

#include "cui-config.h"

#include "cui-call-display.h"
#include "cui-encryption-indicator-priv.h"

#include "cui-call.h"

#include <glib.h>
#include <glib/gi18n-lib.h>
#include <handy.h>
#include <libcallaudio.h>

#define IS_NULL_OR_EMPTY(x)  ((x) == NULL || (x)[0] == '\0')

#define HDY_AVATAR_SIZE_BIG 160
#define HDY_AVATAR_SIZE_DEFAULT 80

/**
 * CuiCallDisplay:
 *
 * A [class@Gtk.Widget] that handles the UI elements of a
 * phone call. It displays the [iface@Cui.Call]'s information and allows
 * actions such as accepting or rejecting the call, hanging up, etc.
 */

enum {
  PROP_0,
  PROP_CALL,
  PROP_LAST_PROP,
};
static GParamSpec *props[PROP_LAST_PROP];

struct _CuiCallDisplay {
  GtkOverlay              parent_instance;

  CuiCall                *call;

  HdyAvatar              *avatar;
  GtkLabel               *primary_contact_info;
  GtkLabel               *secondary_contact_info;
  GtkLabel               *status;

  GtkBox                 *controls;
  GtkBox                 *gsm_controls;
  GtkBox                 *general_controls;
  GtkToggleButton        *speaker;
  GtkToggleButton        *mute;
  GtkButton              *hang_up;
  GtkButton              *answer;
  CuiEncryptionIndicator *encryption_indicator;

  GCancellable           *cancel;
  GtkRevealer            *dial_pad_revealer;
  GtkToggleButton        *dial_pad;
  GtkEntry               *keypad_entry;

  GBinding               *dtmf_bind;
  GBinding               *avatar_icon_bind;
  GBinding               *encryption_bind;

  gboolean                needs_cam_reset; /* cam = Call Audio Mode */
  gboolean                update_status_time;
};

G_DEFINE_TYPE (CuiCallDisplay, cui_call_display, GTK_TYPE_OVERLAY);


/* Just print an error, the main point is that libcallaudio uses async DBus calls */
static void
on_libcallaudio_async_finished (gboolean success, GError *error, gpointer data)
{
  if (!success) {
    g_return_if_fail (error && error->message);
    g_warning ("Failed to select audio mode: %s", error->message);
    g_error_free (error);
  }
}


static void
on_answer_clicked (CuiCallDisplay *self)
{
  g_return_if_fail (CUI_IS_CALL_DISPLAY (self));

  self->update_status_time = FALSE;
  gtk_label_set_label (self->status,
                       _("Accepting call…"));

  gtk_widget_set_sensitive (GTK_WIDGET (self->answer), FALSE);

  cui_call_accept (self->call);
}


static void
on_hang_up_clicked (CuiCallDisplay *self)
{
  g_return_if_fail (CUI_IS_CALL_DISPLAY (self));

  self->update_status_time = FALSE;
  gtk_label_set_label (self->status,
                       _("Hanging up…"));

  gtk_widget_set_sensitive (GTK_WIDGET (self->hang_up), FALSE);

  cui_call_hang_up (self->call);
}


static void
hold_toggled_cb (GtkToggleButton *togglebutton,
                 CuiCallDisplay  *self)
{
}


static void
mute_toggled_cb (GtkToggleButton *togglebutton,
                 CuiCallDisplay  *self)
{
  gboolean want_mute;

  want_mute = gtk_toggle_button_get_active (togglebutton);
  call_audio_mute_mic_async (want_mute, on_libcallaudio_async_finished, NULL);
}


static void
speaker_toggled_cb (GtkToggleButton *togglebutton,
                    CuiCallDisplay  *self)
{
  gboolean want_speaker;

  want_speaker = gtk_toggle_button_get_active (togglebutton);
  call_audio_enable_speaker_async (want_speaker, on_libcallaudio_async_finished, NULL);
}


static void
add_call_clicked_cb (GtkButton      *button,
                     CuiCallDisplay *self)
{
}


static void
hide_dial_pad_clicked_cb (CuiCallDisplay *self)
{
  gtk_revealer_set_reveal_child (self->dial_pad_revealer, FALSE);
}


static void
set_pretty_time (CuiCallDisplay *self)
{
  gdouble elapsed;
  g_autofree char *duration = NULL;

  g_assert (CUI_IS_CALL_DISPLAY (self));
  g_assert (CUI_IS_CALL (self->call));

  elapsed = cui_call_get_active_time (self->call);
  duration = cui_call_format_duration (elapsed);

  gtk_label_set_label (self->status, duration);
}


static void
on_call_state_changed (CuiCallDisplay *self,
                       GParamSpec     *psepc,
                       CuiCall        *call)
{
  GtkStyleContext *hang_up_style;
  CuiCallState state;

  g_return_if_fail (CUI_IS_CALL_DISPLAY (self));
  g_return_if_fail (CUI_IS_CALL (call));

  state = cui_call_get_state (call);

  g_debug ("Call %p changed state to %s",
           call,
           cui_call_state_to_string (state));

  hang_up_style = gtk_widget_get_style_context
                    (GTK_WIDGET (self->hang_up));

  /* if the state changed than the call must be responsive */
  self->update_status_time = TRUE;
  gtk_widget_set_sensitive (GTK_WIDGET (self->answer), TRUE);
  gtk_widget_set_sensitive (GTK_WIDGET (self->hang_up), TRUE);

  /* Widgets and call audio mode*/
  switch (state)
  {
  case CUI_CALL_STATE_INCOMING:
    hdy_avatar_set_size (self->avatar, HDY_AVATAR_SIZE_BIG);

    gtk_widget_hide (GTK_WIDGET (self->controls));
    gtk_widget_show (GTK_WIDGET (self->answer));
    gtk_style_context_remove_class
      (hang_up_style, GTK_STYLE_CLASS_DESTRUCTIVE_ACTION);
    break;

  case CUI_CALL_STATE_ACTIVE:
    hdy_avatar_set_size (self->avatar, HDY_AVATAR_SIZE_DEFAULT);
    G_GNUC_FALLTHROUGH;

  case CUI_CALL_STATE_CALLING:
  case CUI_CALL_STATE_HELD:
    gtk_style_context_add_class
      (hang_up_style, GTK_STYLE_CLASS_DESTRUCTIVE_ACTION);
    gtk_widget_hide (GTK_WIDGET (self->answer));
    gtk_widget_show (GTK_WIDGET (self->controls));

    gtk_widget_set_visible
      (GTK_WIDGET (self->gsm_controls),
      state != CUI_CALL_STATE_CALLING);

    /* TODO Only switch to "call" audio mode for cellular calls */
    call_audio_select_mode_async (CALL_AUDIO_MODE_CALL,
                                  on_libcallaudio_async_finished,
                                  NULL);
    self->needs_cam_reset = TRUE;
    break;

  case CUI_CALL_STATE_DISCONNECTED:
    if (self->needs_cam_reset)
      call_audio_select_mode_async (CALL_AUDIO_MODE_DEFAULT,
                                    on_libcallaudio_async_finished,
                                    NULL);

    gtk_widget_set_sensitive (GTK_WIDGET (self), FALSE);
    break;

  case CUI_CALL_STATE_UNKNOWN:
  default:
    g_warn_if_reached ();
  }

  /* Status text */
  switch (state)
  {
  case CUI_CALL_STATE_ACTIVE:
    set_pretty_time (self);
    break;

  case CUI_CALL_STATE_INCOMING:
  case CUI_CALL_STATE_CALLING:
  case CUI_CALL_STATE_HELD:
  case CUI_CALL_STATE_DISCONNECTED:
    gtk_label_set_label (self->status, cui_call_state_to_string (state));
    break;

  case CUI_CALL_STATE_UNKNOWN:
  default:
    g_warn_if_reached ();
  }
}


static void
on_update_contact_information (CuiCallDisplay *self)
{
  const char *number;
  const char *display_name;
  gboolean show_initials;

  g_assert (CUI_IS_CALL_DISPLAY (self));
  g_assert (CUI_IS_CALL (self->call));

  number = cui_call_get_id (self->call);
  if (IS_NULL_OR_EMPTY (number))
    number = _("Unknown");

  display_name = cui_call_get_display_name (self->call);
  if (IS_NULL_OR_EMPTY (display_name) == FALSE &&
      g_strcmp0 (number, display_name) != 0) {
    show_initials = TRUE;

    gtk_label_set_label (self->primary_contact_info, display_name);
    gtk_label_set_label (self->secondary_contact_info, number);
  } else {
    show_initials = FALSE;

    gtk_label_set_label (self->primary_contact_info, number);
    gtk_label_set_label (self->secondary_contact_info, "");
  }

  hdy_avatar_set_text (self->avatar, display_name);
  hdy_avatar_set_show_initials (self->avatar, show_initials);
}


static void
on_time_updated (CuiCallDisplay *self)
{
  CuiCallState state;

  g_assert (CUI_IS_CALL_DISPLAY (self));
  g_assert (CUI_IS_CALL (self->call));

  state = cui_call_get_state (self->call);
  if (state != CUI_CALL_STATE_ACTIVE &&
      state != CUI_CALL_STATE_HELD) {
    g_warning ("Received timer update, but call is not active!");
    return;
  }

  /* We don't want to overwrite the status text if there
   * is an unfinished operation */
  if (!self->update_status_time)
    return;

  set_pretty_time (self);
}


static void
on_dialpad_revealed (CuiCallDisplay *self)
{
  g_assert (CUI_IS_CALL_DISPLAY (self));

  if (gtk_revealer_get_child_revealed (self->dial_pad_revealer))
    gtk_widget_grab_focus (GTK_WIDGET (self->keypad_entry));
}


static void
reset_ui (CuiCallDisplay *self)
{
  g_assert (CUI_IS_CALL_DISPLAY (self));

  g_debug ("Resetting UI");

  self->update_status_time = TRUE;
  hdy_avatar_set_loadable_icon (self->avatar, NULL);
  hdy_avatar_set_text (self->avatar, "");
  hdy_avatar_set_size (self->avatar, HDY_AVATAR_SIZE_DEFAULT);
  gtk_label_set_label (self->primary_contact_info, "");
  gtk_label_set_label (self->secondary_contact_info, "");
  gtk_label_set_label (self->status, "");
  gtk_widget_show (GTK_WIDGET (self->answer));
  gtk_widget_show (GTK_WIDGET (self->hang_up));
  gtk_widget_show (GTK_WIDGET (self->controls));
  gtk_widget_show (GTK_WIDGET (self->gsm_controls));
  gtk_widget_set_sensitive (GTK_WIDGET (self->answer), TRUE);
  gtk_widget_set_sensitive (GTK_WIDGET (self->hang_up), TRUE);
}


static void
on_call_unrefed (CuiCallDisplay *self,
                 CuiCall        *call)
{
  g_assert (CUI_IS_CALL_DISPLAY (self));

  g_debug ("Dropping call %p", call);

  self->call = NULL;
  self->dtmf_bind = NULL;
  self->avatar_icon_bind = NULL;
  self->encryption_bind = NULL;
  reset_ui (self);
  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_CALL]);
}


static void
cui_call_display_get_property (GObject    *object,
                               guint       property_id,
                               GValue     *value,
                               GParamSpec *pspec)
{
  CuiCallDisplay *self = CUI_CALL_DISPLAY (object);

  switch (property_id) {
  case PROP_CALL:
    g_value_set_object (value, self->call);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}

static void
cui_call_display_set_property (GObject      *object,
                               guint         property_id,
                               const GValue *value,
                               GParamSpec   *pspec)
{
  CuiCallDisplay *self = CUI_CALL_DISPLAY (object);

  switch (property_id) {
  case PROP_CALL:
    cui_call_display_set_call (self, g_value_get_object (value));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
cui_call_display_constructed (GObject *object)
{
  CuiCallDisplay *self = CUI_CALL_DISPLAY (object);

  G_OBJECT_CLASS (cui_call_display_parent_class)->constructed (object);

  g_signal_connect_swapped (self->dial_pad_revealer,
                            "notify::child-revealed",
                            G_CALLBACK (on_dialpad_revealed),
                            self);
}


static void
block_delete_cb (GtkWidget *widget)
{
  g_signal_stop_emission_by_name (widget, "delete-text");
}


static void
insert_text_cb (GtkEditable    *editable,
                gchar          *text,
                gint            length,
                gint           *position,
                CuiCallDisplay *self)
{
  gint end_pos = -1;

  cui_call_send_dtmf (self->call, text);

  // Make sure that new chars are inserted at the end of the input
  *position = end_pos;
  g_signal_handlers_block_by_func (editable,
                                   (gpointer) insert_text_cb, self);
  gtk_editable_insert_text (editable, text, length, &end_pos);
  g_signal_handlers_unblock_by_func (editable,
                                     (gpointer) insert_text_cb, self);

  g_signal_stop_emission_by_name (editable, "insert-text");
}


static void
cui_call_display_dispose (GObject *object)
{
  CuiCallDisplay *self = CUI_CALL_DISPLAY (object);

  if (self->call) {
    g_object_weak_unref (G_OBJECT (self->call), (GWeakNotify) on_call_unrefed, self);
    self->call = NULL;
  }

  G_OBJECT_CLASS (cui_call_display_parent_class)->dispose (object);
}


static void
cui_call_display_class_init (CuiCallDisplayClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->get_property = cui_call_display_get_property;
  object_class->set_property = cui_call_display_set_property;

  object_class->constructed = cui_call_display_constructed;
  object_class->dispose = cui_call_display_dispose;

  /**
   * CuiCallDisplay:call:
   *
   * An opaque handle to a call
   */
  props[PROP_CALL] = g_param_spec_object ("call",
                                          "",
                                          "",
                                          CUI_TYPE_CALL,
                                          G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS |
                                          G_PARAM_EXPLICIT_NOTIFY);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);

  gtk_widget_class_set_template_from_resource (widget_class, "/org/gnome/CallUI/ui/cui-call-display.ui");
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, answer);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, avatar);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, controls);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, dial_pad);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, dial_pad_revealer);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, encryption_indicator);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, general_controls);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, gsm_controls);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, hang_up);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, keypad_entry);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, mute);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, primary_contact_info);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, secondary_contact_info);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, speaker);
  gtk_widget_class_bind_template_child (widget_class, CuiCallDisplay, status);
  gtk_widget_class_bind_template_callback (widget_class, add_call_clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, block_delete_cb);
  gtk_widget_class_bind_template_callback (widget_class, hide_dial_pad_clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, hold_toggled_cb);
  gtk_widget_class_bind_template_callback (widget_class, insert_text_cb);
  gtk_widget_class_bind_template_callback (widget_class, mute_toggled_cb);
  gtk_widget_class_bind_template_callback (widget_class, on_answer_clicked);
  gtk_widget_class_bind_template_callback (widget_class, on_hang_up_clicked);
  gtk_widget_class_bind_template_callback (widget_class, speaker_toggled_cb);

  gtk_widget_class_set_css_name (widget_class, "cui-call-display");
}


static void
cui_call_display_init (CuiCallDisplay *self)
{
  gtk_widget_init_template (GTK_WIDGET (self));

  if (!call_audio_is_inited ()) {
    g_warning ("libcallaudio not initialized");
    gtk_widget_set_sensitive (GTK_WIDGET (self->speaker), FALSE);
    gtk_widget_set_sensitive (GTK_WIDGET (self->mute), FALSE);
  }
}

/**
 * cui_call_display_new:
 * @call: The call this #CuiCalLDisplay handles
 *
 * Creates a new #CuiCallDisplay.
 * Returns: the new #CuiCalLDisplay
 */
CuiCallDisplay *
cui_call_display_new (CuiCall *call)
{
  return g_object_new (CUI_TYPE_CALL_DISPLAY,
                       "call", call,
                       NULL);
}


/**
 * cui_call_display_get_call:
 * @self: The call display
 *
 * Returns the current [iface@CuiCall]
 * Returns: (transfer none) (nullable): The current [iface@CuiCall].
 */
CuiCall *
cui_call_display_get_call (CuiCallDisplay *self)
{
  g_return_val_if_fail (CUI_IS_CALL_DISPLAY (self), NULL);

  return self->call;
}

/**
 * cui_call_display_set_call:
 * @self: The call display
 * @call: (nullable): The current call
 *
 * Set a [iface@CuiCall]. The current call will be removed form the display and the
 * new call displayed instead.
 */
void
cui_call_display_set_call (CuiCallDisplay *self, CuiCall *call)
{
  g_return_if_fail (CUI_IS_CALL_DISPLAY (self));
  g_return_if_fail (CUI_IS_CALL (call) || call == NULL);

  if (self->call == call)
    return;

  if (self->call != NULL) {
    g_object_weak_unref (G_OBJECT (self->call), (GWeakNotify) on_call_unrefed, self);
    g_signal_handlers_disconnect_by_data (self->call, self);
    g_clear_pointer (&self->dtmf_bind, g_binding_unbind);
    g_clear_pointer (&self->avatar_icon_bind, g_binding_unbind);
    g_clear_pointer (&self->encryption_bind, g_binding_unbind);
  }

  self->update_status_time = TRUE;
  self->needs_cam_reset = FALSE;

  self->call = call;
  gtk_widget_set_sensitive (GTK_WIDGET (self), !!self->call);

  if (self->call == NULL) {
    reset_ui (self);
    return;
  }

  g_object_weak_ref (G_OBJECT (call),
                     (GWeakNotify) on_call_unrefed,
                     self);

  g_signal_connect_object (call,
                           "notify::display-name",
                           G_CALLBACK (on_update_contact_information),
                           self,
                           G_CONNECT_SWAPPED);
  on_update_contact_information (self);

  g_signal_connect_object (call, "notify::state",
                           G_CALLBACK (on_call_state_changed),
                           self,
                           G_CONNECT_SWAPPED);
  on_call_state_changed (self, NULL, call);

  g_signal_connect_object (call, "notify::active-time",
                           G_CALLBACK (on_time_updated),
                           self,
                           G_CONNECT_SWAPPED);

  self->dtmf_bind = g_object_bind_property (call,
                                            "can-dtmf",
                                            self->dial_pad,
                                            "sensitive",
                                            G_BINDING_SYNC_CREATE);

  self->avatar_icon_bind = g_object_bind_property (call,
                                                   "avatar-icon",
                                                   self->avatar,
                                                   "loadable-icon",
                                                   G_BINDING_SYNC_CREATE);
  self->encryption_bind = g_object_bind_property (call,
                                                  "encrypted",
                                                  self->encryption_indicator,
                                                  "encrypted",
                                                  G_BINDING_SYNC_CREATE);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_CALL]);
}
