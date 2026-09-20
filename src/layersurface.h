/*
 * Copyright (C) 2018 Purism SPC
 *               2026 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define PHOSH_TYPE_LAYER_SURFACE                 (phosh_layer_surface_get_type ())

G_DECLARE_DERIVABLE_TYPE (PhoshLayerSurface, phosh_layer_surface, PHOSH, LAYER_SURFACE, GtkPlain)

/* Must match the Waland protocol values */
typedef enum {
  PHOSH_LAYER_SURFACE_ANCHOR_NONE = 0,
  PHOSH_LAYER_SURFACE_ANCHOR_TOP = (1 << 0),
  PHOSH_LAYER_SURFACE_ANCHOR_BOTTOM = (1 << 1),
  PHOSH_LAYER_SURFACE_ANCHOR_LEFT = (1 << 2),
  PHOSH_LAYER_SURFACE_ANCHOR_RIGHT = (1 << 3),
} PhoshLayerSurfaceAnchor;

/* Must match the Wayland protocol values */
typedef enum {
  PHOSH_LAYER_SURFACE_LAYER_BACKGROUND = 0,
  PHOSH_LAYER_SURFACE_LAYER_BOTTOM = 1,
  PHOSH_LAYER_SURFACE_LAYER_TOP = 2,
  PHOSH_LAYER_SURFACE_LAYER_OVERLAY = 3,
} PhoshLayerSurfaceLayer;

/**
 * PhoshLayerSurfaceClass
 * @parent_class: The parent class
 * @configured: invoked when layer surface is configured
 */
struct _PhoshLayerSurfaceClass
{
  GtkWindowClass parent_class;

  /* Signals
   */
  void (*configured)   (PhoshLayerSurface    *self);

  /* Padding for future expansion */
  void                 (*_phosh_reserved1) (void);
  void                 (*_phosh_reserved2) (void);
  void                 (*_phosh_reserved3) (void);
  void                 (*_phosh_reserved4) (void);
  void                 (*_phosh_reserved5) (void);
  void                 (*_phosh_reserved6) (void);
  void                 (*_phosh_reserved7) (void);
  void                 (*_phosh_reserved8) (void);
  void                 (*_phosh_reserved9) (void);
};

void
phosh_layer_surface_set_child (PhoshLayerSurface *self, GtkWidget *child);

G_END_DECLS
