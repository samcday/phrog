/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */
#pragma once

#include <glib.h>

G_BEGIN_DECLS

void cui_init (gboolean init_callaudio);
void cui_uninit (void);

G_END_DECLS
