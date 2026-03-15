/*
 * Copyright (C) 2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Gotam Gorabh <gautamy672@gmail.com>
 */

 #pragma once

 #include "quick-setting.h"

G_BEGIN_DECLS

#define PHOSH_TYPE_LOCATION_QUICK_SETTING phosh_location_quick_setting_get_type ()
G_DECLARE_FINAL_TYPE (PhoshLocationQuickSetting,
                      phosh_location_quick_setting,
                      PHOSH, LOCATION_QUICK_SETTING, PhoshQuickSetting)

G_END_DECLS
