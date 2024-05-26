/*
 * Copyright (C) 2022 Purism SPC
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

guint       gm_timeout_add_seconds_once_full (int             priority,
					      gulong          seconds,
					      GSourceOnceFunc function,
					      gpointer        data,
					      GDestroyNotify  notify);
guint       gm_timeout_add_seconds_once      (int             seconds,
					      GSourceOnceFunc function,
					      gpointer        data);

G_END_DECLS
