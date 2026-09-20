/*
 * Copyright (C) 2025 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include <handy.h>

#define PHOSH_INTERVAL_ROW_INFINITY_VALUE G_MAXUINT32

G_BEGIN_DECLS

#define PHOSH_TYPE_INTERVAL_ROW (phosh_interval_row_get_type ())

G_DECLARE_FINAL_TYPE (PhoshIntervalRow, phosh_interval_row, PHOSH, INTERVAL_ROW, HdyActionRow)

PhoshIntervalRow *phosh_interval_row_new (uint value, gboolean selected);
uint              phosh_interval_row_get_value (PhoshIntervalRow *self);
void              phosh_interval_row_set_selected (PhoshIntervalRow *self, gboolean selected);

G_END_DECLS
