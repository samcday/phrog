/*
 * Copyright (C) 2018 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include "layersurface.h"
/* TODO: We use the enum constants from here, use glib-mkenums */
#include "wlr-layer-shell-unstable-v1-client-protocol.h"

G_BEGIN_DECLS

GtkWidget *phosh_layer_surface_new (gpointer wl_output);
struct     zwlr_layer_surface_v1 *phosh_layer_surface_get_layer_surface(PhoshLayerSurface *self);
struct     wl_surface            *phosh_layer_surface_get_wl_surface(PhoshLayerSurface *self);
void                              phosh_layer_surface_set_size(PhoshLayerSurface *self,
                                                               int width,
                                                               int height);
void                              phosh_layer_surface_set_margins(PhoshLayerSurface *self,
                                                                  int top,
                                                                  int right,
                                                                  int bottom,
                                                                  int left);
void                              phosh_layer_surface_set_exclusive_zone(PhoshLayerSurface *self,
                                                                         int zone);
void                              phosh_layer_surface_set_kbd_interactivity(PhoshLayerSurface *self,
                                                                            gboolean interactivity);
PhoshLayerSurfaceLayer            phosh_layer_surface_get_layer (PhoshLayerSurface *self);
void                              phosh_layer_surface_set_layer (PhoshLayerSurface     *self,
                                                                 PhoshLayerSurfaceLayer layer);
void                              phosh_layer_surface_wl_surface_commit (PhoshLayerSurface *self);
void                              phosh_layer_surface_get_margins       (PhoshLayerSurface *self,
                                                                         int               *top,
                                                                         int               *right,
                                                                         int               *bottom,
                                                                         int               *left);
int                               phosh_layer_surface_get_configured_width  (PhoshLayerSurface *self);
int                               phosh_layer_surface_get_configured_height (PhoshLayerSurface *self);
void                              phosh_layer_surface_set_alpha (PhoshLayerSurface *self,
                                                                 double             alpha);
void                              phosh_layer_surface_set_stacked_above (PhoshLayerSurface *self,
                                                                         PhoshLayerSurface *target);
void                              phosh_layer_surface_set_stacked_below (PhoshLayerSurface *self,
                                                                         PhoshLayerSurface *target);
gpointer                          phosh_layer_surface_get_wl_output (PhoshLayerSurface *self);
const char *                      phosh_layer_surface_get_namespace (PhoshLayerSurface *self);
void                              phosh_layer_surface_destroy (PhoshLayerSurface *self);

G_END_DECLS
