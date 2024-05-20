/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#if !GTK_CHECK_VERSION (3, 22, 0)
# error "libhandy requires gtk+-3.0 >= 3.22.0"
#endif

#if !GLIB_CHECK_VERSION (2, 50, 0)
# error "libhandy requires glib-2.0 >= 2.50.0"
#endif

#define _CALL_UI_INSIDE

#include "cui-call.h"
#include "cui-call-display.h"
#include "cui-dialpad.h"
#include "cui-keypad.h"
#include "cui-enums.h"
#include "cui-main.h"

#undef _CALL_UI_INSIDE

G_END_DECLS
