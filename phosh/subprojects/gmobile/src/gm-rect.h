/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include <glib-object.h>

#pragma once

#if !defined(_GMOBILE_INSIDE) && !defined(GMOBILE_COMPILATION)
#error "Only <gmobile.h> can be included directly."
#endif

G_BEGIN_DECLS

typedef struct _GmRect {
  int x, y;
  int width, height;
} GmRect;

GType gm_rect_get_type (void) G_GNUC_CONST;

#define GM_TYPE_RECT (gm_rect_get_type ())

G_END_DECLS
