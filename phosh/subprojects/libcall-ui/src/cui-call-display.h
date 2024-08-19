/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#pragma once

#include "cui-call.h"

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define CUI_TYPE_CALL_DISPLAY (cui_call_display_get_type ())

G_DECLARE_FINAL_TYPE (CuiCallDisplay, cui_call_display, CUI, CALL_DISPLAY, GtkOverlay);

CuiCallDisplay *cui_call_display_new        (CuiCall *call);
void            cui_call_display_set_call   (CuiCallDisplay *self,
                                             CuiCall        *call);
CuiCall        *cui_call_display_get_call   (CuiCallDisplay *self);

G_END_DECLS
