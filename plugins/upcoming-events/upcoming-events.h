/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */


#include <gtk/gtk.h>

#pragma once

G_BEGIN_DECLS

#define PHOSH_TYPE_UPCOMING_EVENTS (phosh_upcoming_events_get_type ())
G_DECLARE_FINAL_TYPE (PhoshUpcomingEvents, phosh_upcoming_events, PHOSH, UPCOMING_EVENTS, GtkBox)

G_END_DECLS
