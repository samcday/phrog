/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#pragma once

#if !defined(_GMOBILE_INSIDE) && !defined(GMOBILE_COMPILATION)
#error "Only <gmobile.h> can be included directly."
#endif

#include <glib.h>

G_BEGIN_DECLS

gboolean               gm_svg_path_get_bounding_box (const char *path,
                                                     int        *x1,
                                                     int        *x2,
                                                     int        *y1,
                                                     int        *y2,
                                                     GError   **err);

G_END_DECLS
