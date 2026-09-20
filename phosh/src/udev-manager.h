/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "dbus/login1-session-dbus.h"

#include <glib-object.h>

#include <gudev/gudev.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_UDEV_MANAGER (phosh_udev_manager_get_type ())

G_DECLARE_FINAL_TYPE (PhoshUdevManager, phosh_udev_manager, PHOSH, UDEV_MANAGER, GObject)

PhoshUdevManager *     phosh_udev_manager_get_default (void);
GUdevDevice *          phosh_udev_manager_find_backlight (PhoshUdevManager *self,
                                                          const char *connector_name);
GList *                phosh_udev_manager_find_torches (PhoshUdevManager *self, GError **err);
PhoshDBusLoginSession *phosh_udev_manager_get_session_proxy (PhoshUdevManager *self);

G_END_DECLS
