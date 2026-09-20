/*
 * Copyright (C) 2020 Purism SPC
 *               2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
#pragma once

#include "dbus/geoclue-agent-dbus.h"
#include <glib-object.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_LOCATION_MANAGER     (phosh_location_manager_get_type ())
G_DECLARE_FINAL_TYPE (PhoshLocationManager, phosh_location_manager, PHOSH, LOCATION_MANAGER,
                      PhoshDBusGeoClue2AgentSkeleton)

PhoshLocationManager * phosh_location_manager_new (void);

G_END_DECLS
