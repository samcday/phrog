/*
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Copyright (C) 2021 Purism SPC
 *
 * Based on calls-encryption-indicator by
 * Adrien Plazas <adrien.plazas@puri.sm>
 */

#include "cui-config.h"

#include "cui-encryption-indicator-priv.h"

#include <glib/gi18n-lib.h>

struct _CuiEncryptionIndicator {
  GtkStack parent_instance;

  GtkBox  *is_not_encrypted;
  GtkBox  *is_encrypted;
};

G_DEFINE_TYPE (CuiEncryptionIndicator, cui_encryption_indicator, GTK_TYPE_STACK);

enum {
  PROP_0,
  PROP_ENCRYPTED,
  PROP_LAST_PROP,
};
static GParamSpec *props[PROP_LAST_PROP];


void
cui_encryption_indicator_set_encrypted (CuiEncryptionIndicator *self,
                                        gboolean                encrypted)
{
  g_return_if_fail (CUI_IS_ENCRYPTION_INDICATOR (self));

  encrypted = !!encrypted;

  gtk_stack_set_visible_child (
    GTK_STACK (self),
    GTK_WIDGET (encrypted ? self->is_encrypted : self->is_not_encrypted));
}


gboolean
cui_encryption_indicator_get_encrypted (CuiEncryptionIndicator *self)
{
  g_return_val_if_fail (CUI_IS_ENCRYPTION_INDICATOR (self), FALSE);

  return gtk_stack_get_visible_child (GTK_STACK (self)) == GTK_WIDGET (self->is_encrypted);
}


static void
cui_encryption_indicator_init (CuiEncryptionIndicator *self)
{
  gtk_widget_init_template (GTK_WIDGET (self));
}


static void
set_property (GObject      *object,
              guint         property_id,
              const GValue *value,
              GParamSpec   *pspec)
{
  CuiEncryptionIndicator *self = CUI_ENCRYPTION_INDICATOR (object);

  switch (property_id) {
  case PROP_ENCRYPTED:
    cui_encryption_indicator_set_encrypted (self, g_value_get_boolean (value));
    break;

  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
get_property (GObject    *object,
              guint       property_id,
              GValue     *value,
              GParamSpec *pspec)
{
  CuiEncryptionIndicator *self = CUI_ENCRYPTION_INDICATOR (object);

  switch (property_id) {
  case PROP_ENCRYPTED:
    g_value_set_boolean (value, cui_encryption_indicator_get_encrypted (self));
    break;

  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
cui_encryption_indicator_class_init (CuiEncryptionIndicatorClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->set_property = set_property;
  object_class->get_property = get_property;

  props[PROP_ENCRYPTED] =
    g_param_spec_boolean ("encrypted",
                          "Encrypted",
                          "The party participating in the call",
                          FALSE,
                          G_PARAM_READWRITE);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);
  gtk_widget_class_set_template_from_resource (widget_class, "/org/gnome/CallUI/ui/cui-encryption-indicator.ui");
  gtk_widget_class_bind_template_child (widget_class, CuiEncryptionIndicator, is_not_encrypted);
  gtk_widget_class_bind_template_child (widget_class, CuiEncryptionIndicator, is_encrypted);
}
