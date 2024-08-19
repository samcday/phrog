/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Thomas Booker <tw.booker@outlook.com>
 *
 * Based on calls-new-call-box by
 * Adrien Plazas <adrien.plazas@puri.sm>
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define CUI_TYPE_DIALPAD (cui_dialpad_get_type ())

G_DECLARE_FINAL_TYPE (CuiDialpad, cui_dialpad, CUI, DIALPAD, GtkBox);

CuiDialpad *cui_dialpad_new        (void);

G_END_DECLS
