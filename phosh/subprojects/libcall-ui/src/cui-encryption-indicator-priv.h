/*
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Copyright (C) 2021 Purism SPC
 *
 * Based on calls-encryption-indicator by
 * Author: Adrien Plazas <adrien.plazas@puri.sm>
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define CUI_TYPE_ENCRYPTION_INDICATOR (cui_encryption_indicator_get_type ())

G_DECLARE_FINAL_TYPE (CuiEncryptionIndicator, cui_encryption_indicator, CUI, ENCRYPTION_INDICATOR, GtkStack);

void     cui_encryption_indicator_set_encrypted (CuiEncryptionIndicator *self,
                                                 gboolean                encrypted);
gboolean cui_encryption_indicator_get_encrypted (CuiEncryptionIndicator *self);

G_END_DECLS
