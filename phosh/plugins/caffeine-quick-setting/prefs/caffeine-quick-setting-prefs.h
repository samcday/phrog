/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include <adwaita.h>

#pragma once

G_BEGIN_DECLS

#define PHOSH_TYPE_CAFFEINE_QUICK_SETTING_PREFS (phosh_caffeine_quick_setting_prefs_get_type ())
G_DECLARE_FINAL_TYPE (PhoshCaffeineQuickSettingPrefs,
                      phosh_caffeine_quick_setting_prefs,
                      PHOSH, CAFFEINE_QUICK_SETTING_PREFS,
                      AdwPreferencesDialog)

G_END_DECLS
