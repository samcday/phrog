/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * based HdyKeypad which is
 * Copyright (C) 2019 Purism SPC
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define CUI_TYPE_KEYPAD (cui_keypad_get_type ())

G_DECLARE_FINAL_TYPE (CuiKeypad, cui_keypad, CUI, KEYPAD, GtkBin);

GtkWidget       *cui_keypad_new                     (gboolean symbols_visible,
                                                     gboolean letters_visible);

void             cui_keypad_set_row_spacing         (CuiKeypad *self,
                                                     guint      spacing);

guint            cui_keypad_get_row_spacing         (CuiKeypad *self);

void             cui_keypad_set_column_spacing      (CuiKeypad *self,
                                                     guint      spacing);

guint            cui_keypad_get_column_spacing      (CuiKeypad *self);

void             cui_keypad_set_letters_visible     (CuiKeypad *self,
                                                     gboolean   letters_visible);

gboolean         cui_keypad_get_letters_visible     (CuiKeypad *self);

void             cui_keypad_set_symbols_visible     (CuiKeypad *self,
                                                     gboolean   symbols_visible);

gboolean         cui_keypad_get_symbols_visible     (CuiKeypad *self);

void             cui_keypad_set_entry               (CuiKeypad *self,
                                                     GtkEntry  *entry);

GtkEntry        *cui_keypad_get_entry               (CuiKeypad *self);

void             cui_keypad_set_start_action        (CuiKeypad *self,
                                                     GtkWidget *start_action);

GtkWidget       *cui_keypad_get_start_action        (CuiKeypad *self);

void             cui_keypad_set_end_action          (CuiKeypad *self,
                                                     GtkWidget *end_action);

GtkWidget       *cui_keypad_get_end_action          (CuiKeypad *self);

G_END_DECLS
