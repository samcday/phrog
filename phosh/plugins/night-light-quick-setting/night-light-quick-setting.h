/*
 * Copyright (C) 2024 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#pragma once

#include "quick-setting.h"

G_BEGIN_DECLS

#define PHOSH_TYPE_NIGHT_LIGHT_QUICK_SETTING phosh_night_light_quick_setting_get_type ()

G_DECLARE_FINAL_TYPE (PhoshNightLightQuickSetting,
                      phosh_night_light_quick_setting,
                      PHOSH, NIGHT_LIGHT_QUICK_SETTING, PhoshQuickSetting)

G_END_DECLS
