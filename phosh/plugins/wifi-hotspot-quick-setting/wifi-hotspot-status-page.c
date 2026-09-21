/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Arun Mani J <arun.mani@tether.to>
 */

#include "plugin-shell.h"
#include "status-page-placeholder.h"
#include "wifi-hotspot-status-page.h"

#include <qrcodegen.h>

#define BYTES_PER_R8G8B8 3
#define QR_CODE_SIZE 128

/**
 * PhoshWifiHotspotStatusPage:
 *
 * A status-page to show password of Wi-Fi hotspot.
 *
 * The code to display QR is taken from GNOME Control Center.
 */

struct _PhoshWifiHotspotStatusPage {
  PhoshStatusPage   parent;

  GtkEntry         *entry;
  GtkImage         *image;
  PhoshStatusPagePlaceholder *placeholder;
  GtkLabel         *ssid;
  GtkStack         *stack;
  GtkButton        *turn_on_btn;

  GCancellable     *cancel;
  PhoshWifiManager *wifi;
};

G_DEFINE_TYPE (PhoshWifiHotspotStatusPage, phosh_wifi_hotspot_status_page, PHOSH_TYPE_STATUS_PAGE);


static void
fill_pixel (GByteArray *array, guint8 value, int pixel_size)
{
  guint i;

  for (i = 0; i < pixel_size; i++)
    {
      g_byte_array_append (array, &value, 1); /* R */
      g_byte_array_append (array, &value, 1); /* G */
      g_byte_array_append (array, &value, 1); /* B */
    }
}


static GdkPaintable *
qr_from_text (const char *text, int size, int scale)
{
  uint8_t qr_code[qrcodegen_BUFFER_LEN_FOR_VERSION (qrcodegen_VERSION_MAX)];
  uint8_t temp_buf[qrcodegen_BUFFER_LEN_FOR_VERSION (qrcodegen_VERSION_MAX)];
  g_autoptr (GBytes) bytes = NULL;
  GByteArray *qr_matrix;
  int pixel_size, qr_size, total_size;
  int column, row, i;
  gboolean success = FALSE;
  GdkTexture *texture;

  g_return_val_if_fail (size > 0, NULL);

  success = qrcodegen_encodeText (text,
                                  temp_buf,
                                  qr_code,
                                  qrcodegen_Ecc_LOW,
                                  qrcodegen_VERSION_MIN,
                                  qrcodegen_VERSION_MAX,
                                  qrcodegen_Mask_AUTO,
                                  FALSE);

  if (!success)
    return NULL;

  qr_size = qrcodegen_getSize (qr_code);
  pixel_size = MAX (1, size / (qr_size));
  total_size = qr_size * pixel_size;
  qr_matrix = g_byte_array_sized_new (total_size * total_size * pixel_size * BYTES_PER_R8G8B8);

  for (column = 0; column < total_size; column++)
    {
      for (i = 0; i < pixel_size; i++)
        {
          for (row = 0; row < total_size / pixel_size; row++)
            {
              if (qrcodegen_getModule (qr_code, column, row))
                fill_pixel (qr_matrix, 0x00, pixel_size);
              else
                fill_pixel (qr_matrix, 0xff, pixel_size);
            }
        }
    }

  bytes = g_byte_array_free_to_bytes (qr_matrix);

  texture = gdk_memory_texture_new (total_size,
                                    total_size,
                                    GDK_MEMORY_R8G8B8,
                                    bytes,
                                    total_size * BYTES_PER_R8G8B8);

  return GDK_PAINTABLE (texture);
}


static char *
escape_string (const char *str, gboolean quote)
{
  GString *string;
  const char *next;

  if (!str)
    return NULL;

  string = g_string_new ("");
  if (quote)
    g_string_append_c (string, '"');

  while ((next = strpbrk (str, "\\;,:\""))) {
    g_string_append_len (string, str, next - str);
    g_string_append_c (string, '\\');
    g_string_append_c (string, *next);
    str = next + 1;
  }

  g_string_append (string, str);
  if (quote)
    g_string_append_c (string, '"');

  return g_string_free (string, FALSE);
}


static const char *
get_connection_security_type (NMConnection *c)
{
  NMSettingWirelessSecurity *setting;
  const char *key_mgmt;

  g_return_val_if_fail (c, "nopass");

  setting = nm_connection_get_setting_wireless_security (c);

  if (!setting)
    return "nopass";

  key_mgmt = nm_setting_wireless_security_get_key_mgmt (setting);

  /* No IEEE 802.1x */
  if (g_strcmp0 (key_mgmt, "none") == 0)
    return "WEP";

  if (g_strcmp0 (key_mgmt, "wpa-psk") == 0)
    return "WPA";

  if (g_strcmp0 (key_mgmt, "sae") == 0)
    return "SAE";

  return "nopass";
}


static char *
get_wifi_password (NMConnection *c)
{
  NMSettingWirelessSecurity *setting;
  const char *sec_type, *password;
  int wep_index;

  sec_type = get_connection_security_type (c);
  setting = nm_connection_get_setting_wireless_security (c);

  if (g_str_equal (sec_type, "nopass"))
    return NULL;

  if (g_str_equal (sec_type, "WEP")) {
    wep_index = nm_setting_wireless_security_get_wep_tx_keyidx (setting);
    password = nm_setting_wireless_security_get_wep_key (setting, wep_index);
  } else {
    password = nm_setting_wireless_security_get_psk (setting);
  }

  return g_strdup (password);
}


/* Generate a string representing the connection
 * An example generated text:
 *     WIFI:S:ssid;T:WPA;P:my-valid-pass;H:true;
 * Where,
 *   S = ssid, T = security, P = password, H = hidden (Optional)
 *
 * See https://github.com/zxing/zxing/wiki/Barcode-Contents#wi-fi-network-config-android-ios-11
 */
static char *
get_qr_string_for_connection (NMConnection *c)
{
  NMSettingWireless *setting;
  g_autofree char *ssid_text = NULL;
  g_autofree char *escaped_ssid = NULL;
  g_autofree char *password_str = NULL;
  g_autofree char *escaped_password = NULL;
  GString *string;
  GBytes *ssid;
  gboolean hidden;

  setting = nm_connection_get_setting_wireless (c);
  ssid = nm_setting_wireless_get_ssid (setting);

  if (!ssid)
    return NULL;

  string = g_string_new ("WIFI:S:");

  /* SSID */
  ssid_text = nm_utils_ssid_to_utf8 (g_bytes_get_data (ssid, NULL),
                                     g_bytes_get_size (ssid));
  escaped_ssid = escape_string (ssid_text, FALSE);
  g_string_append (string, escaped_ssid);
  g_string_append_c (string, ';');

  /* Security type */
  g_string_append (string, "T:");
  g_string_append (string, get_connection_security_type (c));
  g_string_append_c (string, ';');

  /* Password */
  g_string_append (string, "P:");
  password_str = get_wifi_password (c);
  escaped_password = escape_string (password_str, FALSE);
  if (escaped_password)
    g_string_append (string, escaped_password);
  g_string_append_c (string, ';');

  /* WiFi Hidden */
  hidden = nm_setting_wireless_get_hidden (setting);
  if (hidden)
    g_string_append (string, "H:true");
  g_string_append_c (string, ';');

  return g_string_free (string, FALSE);
}


static void
on_secrets_ready (GObject *object, GAsyncResult *result, gpointer data)
{
  PhoshWifiHotspotStatusPage *self = data;
  NMConnection *conn = NM_CONNECTION (object);
  g_autoptr (GError) error = NULL;
  g_autoptr (GVariant) variant = NULL;
  g_autofree char *cnx_str = NULL;
  g_autofree char *password = NULL;
  g_autoptr (GdkPaintable) paintable = NULL;
  int scale;

  variant = nm_remote_connection_get_secrets_finish (NM_REMOTE_CONNECTION (conn), result, &error);
  if (variant == NULL) {
    g_warning ("Unable to fetch secrets: %s", error->message);
    gtk_image_set_from_icon_name (self->image, "face-sad-symbolic");
    gtk_editable_set_text (GTK_EDITABLE (self->entry), "");
    return;
  }
  if (!nm_connection_update_secrets (conn,
                                     NM_SETTING_WIRELESS_SECURITY_SETTING_NAME,
                                     variant,
                                     &error)) {
    g_warning ("Unable to set secrets: %s", error->message);
    gtk_image_set_from_icon_name (self->image, "face-sad-symbolic");
    gtk_editable_set_text (GTK_EDITABLE (self->entry), "");
    return;
  }

  cnx_str = get_qr_string_for_connection (conn);
  scale = gtk_widget_get_scale_factor (GTK_WIDGET (self->image));
  paintable = qr_from_text (cnx_str, QR_CODE_SIZE, scale);
  password = get_wifi_password (conn);
  gtk_image_set_from_paintable (self->image, paintable);
  gtk_editable_set_text (GTK_EDITABLE (self->entry), password);
  nm_connection_clear_secrets (conn);
}


static void
setup_hotspot_page (PhoshWifiHotspotStatusPage *self)
{
  NMActiveConnection *conn = phosh_wifi_manager_get_active_connection (self->wifi);
  NMRemoteConnection *remote = nm_active_connection_get_connection (conn);

  nm_remote_connection_get_secrets_async (NM_REMOTE_CONNECTION (remote),
                                          NM_SETTING_WIRELESS_SECURITY_SETTING_NAME,
                                          self->cancel,
                                          on_secrets_ready,
                                          self);
}


static void
on_wifi_notify (PhoshWifiHotspotStatusPage *self)
{
  gboolean wifi_absent = !phosh_wifi_manager_get_present (self->wifi);
  gboolean wifi_disabled = !phosh_wifi_manager_get_enabled (self->wifi);
  gboolean hotspot_disabled = !phosh_wifi_manager_is_hotspot_master (self->wifi);
  const char *icon_name;

  if (wifi_absent)
    icon_name = "network-wireless-hardware-disabled-symbolic";
  else if (wifi_disabled)
    icon_name = "network-wireless-disabled-symbolic";
  else
    icon_name = "network-wireless-hotspot-disabled-symbolic";

  phosh_status_page_placeholder_set_icon_name (self->placeholder, icon_name);
  gtk_widget_set_visible (GTK_WIDGET (self->turn_on_btn), !wifi_absent);

  if (hotspot_disabled) {
    gtk_stack_set_visible_child_name (self->stack, "empty_state");
  } else {
    gtk_stack_set_visible_child_name (self->stack, "hotspot_enabled");
    setup_hotspot_page (self);
  }
}


static void
on_icon_press (PhoshWifiHotspotStatusPage *self)
{
  gboolean visibility = gtk_entry_get_visibility (self->entry);
  const char *icon_name;

  if (visibility)
    icon_name = "view-reveal-symbolic";
  else
    icon_name = "view-conceal-symbolic";

  gtk_entry_set_visibility (self->entry, !visibility);
  gtk_entry_set_icon_from_icon_name (self->entry, GTK_ENTRY_ICON_SECONDARY, icon_name);
}


static void
on_turn_on_clicked (PhoshWifiHotspotStatusPage *self)
{
  gboolean wifi_disabled = !phosh_wifi_manager_get_enabled (self->wifi);

  if (wifi_disabled)
    phosh_wifi_manager_set_enabled (self->wifi, TRUE);
  else
    phosh_wifi_manager_set_hotspot_master (self->wifi, TRUE);
}


static void
phosh_wifi_hotspot_status_page_dispose (GObject *object)
{
  PhoshWifiHotspotStatusPage *self = PHOSH_WIFI_HOTSPOT_STATUS_PAGE (object);

  g_cancellable_cancel (self->cancel);
  g_clear_object (&self->cancel);

  if (self->wifi)
    g_signal_handlers_disconnect_by_data (self->wifi, self);

  G_OBJECT_CLASS (phosh_wifi_hotspot_status_page_parent_class)->dispose (object);
}


static void
phosh_wifi_hotspot_status_page_class_init (PhoshWifiHotspotStatusPageClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->dispose = phosh_wifi_hotspot_status_page_dispose;

  gtk_widget_class_set_template_from_resource (widget_class,
                                               "/mobi/phosh/plugins/wifi-hotspot-quick-setting/status-page.ui");

  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, entry);
  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, image);
  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, placeholder);
  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, ssid);
  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, stack);
  gtk_widget_class_bind_template_child (widget_class, PhoshWifiHotspotStatusPage, turn_on_btn);
  gtk_widget_class_bind_template_callback (widget_class, on_turn_on_clicked);
  gtk_widget_class_bind_template_callback (widget_class, on_icon_press);

  gtk_widget_class_set_css_name (widget_class, "phosh-wifi-hotspot-status-page");
}


static void
phosh_wifi_hotspot_status_page_init (PhoshWifiHotspotStatusPage *self)
{
  gtk_widget_init_template (GTK_WIDGET (self));

  self->cancel = g_cancellable_new ();
  self->wifi = phosh_shell_get_wifi_manager (phosh_shell_get_default ());

  g_return_if_fail (PHOSH_IS_WIFI_MANAGER (self->wifi));

  g_object_connect (self->wifi,
                    "swapped-object-signal::notify::present",
                    G_CALLBACK (on_wifi_notify), self,
                    "swapped-object-signal::notify::enabled",
                    G_CALLBACK (on_wifi_notify), self,
                    "swapped-object-signal::notify::is-hotspot-master",
                    G_CALLBACK (on_wifi_notify), self,
                    NULL);
  g_object_bind_property (self->wifi, "ssid", self->ssid, "label", G_BINDING_SYNC_CREATE);
}


GtkWidget *
phosh_wifi_hotspot_status_page_new (void)
{
  return g_object_new (PHOSH_TYPE_WIFI_HOTSPOT_STATUS_PAGE, NULL);
}
