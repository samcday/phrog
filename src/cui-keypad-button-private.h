/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * based HdyKeypad which is
 * Copyright (C) 2019 Purism SPC
 */

#pragma once

#if !defined(_HANDY_INSIDE) && !defined(HANDY_COMPILATION)
#endif

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define CUI_TYPE_KEYPAD_BUTTON (cui_keypad_button_get_type())

G_DECLARE_FINAL_TYPE (CuiKeypadButton, cui_keypad_button, CUI, KEYPAD_BUTTON, GtkButton)

struct _CuiKeypadButtonClass
{
  GtkButtonClass parent_class;
};

GtkWidget   *cui_keypad_button_new                   (const gchar     *symbols);
gchar        cui_keypad_button_get_digit             (CuiKeypadButton *self);
const gchar *cui_keypad_button_get_symbols           (CuiKeypadButton *self);
void         cui_keypad_button_show_symbols          (CuiKeypadButton *self,
                                                      gboolean         visible);

G_END_DECLS
