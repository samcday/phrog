/*
 * Copyright (C) 2022 Purism SPC
 *               2022-2024 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#pragma once

#ifndef GMOBILE_USE_UNSTABLE_API
#error    gmobile is unstable API. You must define GMOBILE_USE_UNSTABLE_API before including gmobile.h
#endif

#define _GMOBILE_INSIDE

#include "gm-config.h"
#include "gm-cutout.h"
#include "gm-device-info.h"
#include "gm-device-tree.h"
#include "gm-display-panel.h"
#include "gm-error.h"
#include "gm-main.h"
#include "gm-timeout.h"
#include "gm-util.h"
