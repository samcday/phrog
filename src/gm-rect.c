/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-rect.h"

/**
 * GmRect:
 * @x: x coordindate of the upper, left corner
 * @y: y coordindate of the upper, left corner
 * @width: the width of the rectangle
 * @height: the height of the rectangle
 *
 * A rectangle.
 *
 * Similar to GdkRectangle but we don't want to pull in gtk/gdk.
 *
 * Since: 0.0.1
 */

static GmRect *
gm_rect_copy (const GmRect *self)
{
  GmRect *copy = g_new (GmRect, 1);
  *copy = *self;

  return copy;
}


/* TODO: transform from/to GdkRectangle */
G_DEFINE_BOXED_TYPE (GmRect, gm_rect, gm_rect_copy, g_free);
