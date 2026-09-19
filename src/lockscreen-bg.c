/*
 * Copyright (C) 2025 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define G_LOG_DOMAIN "phosh-lockscreen-bg"

#include "phosh-config.h"

#include "shell-priv.h"
#include "lockscreen-bg.h"
#include "style-manager.h"
#include "util.h"

#include <gmobile.h>

#include <gdesktop-enums.h>

#include <gtk/gtk.h>

/**
 * PhoshLockscreenBg:
 *
 * The lockscreen's background
 */

struct _PhoshLockscreenBg {
  PhoshLayerSurface     parent;

  GdkPixbuf            *pixbuf;
  PhoshBackgroundImage *bg_image;

  gboolean              configured;
  gboolean              use_background;
};
G_DEFINE_TYPE (PhoshLockscreenBg, phosh_lockscreen_bg, PHOSH_TYPE_LAYER_SURFACE)


static void
update_image (PhoshLockscreenBg *self)
{
  int width, height;

  if (!self->configured)
    return;

  width = phosh_layer_surface_get_configured_width (PHOSH_LAYER_SURFACE (self));
  height = phosh_layer_surface_get_configured_height (PHOSH_LAYER_SURFACE (self));

  g_return_if_fail (width > 0 && height > 0);

  g_debug ("Scaling lockscreen background %p to %dx%d", self, width, height);

  g_clear_object (&self->pixbuf);
  if (self->bg_image) {
    GdkPixbuf *pixbuf = phosh_background_image_get_pixbuf (self->bg_image);
    self->pixbuf = phosh_utils_pixbuf_scale_to_min (pixbuf, width, height);
  }

  gtk_widget_queue_draw (GTK_WIDGET (self));
}


static void
phosh_lockscreen_bg_configured (PhoshLayerSurface *layer_surface)
{
  PhoshLockscreenBg *self = PHOSH_LOCKSCREEN_BG (layer_surface);

  PHOSH_LAYER_SURFACE_CLASS (phosh_lockscreen_bg_parent_class)->configured (layer_surface);

  self->configured = TRUE;
  update_image (self);
}


static void
phosh_lockscreen_bg_snapshot (GtkWidget *widget, GtkSnapshot *snapshot)
{
  PhoshLockscreenBg *self = PHOSH_LOCKSCREEN_BG (widget);
  graphene_rect_t bounds;
  int x = 0, y = 0, width, height;
  cairo_t *cr;

  g_return_if_fail (PHOSH_IS_LOCKSCREEN_BG (self));

  if (!self->configured)
    return;

  bounds.origin.x = 0;
  bounds.origin.y = 0;
  bounds.size.width = gtk_widget_get_width (GTK_WIDGET (self));
  bounds.size.height = gtk_widget_get_height (GTK_WIDGET (self));

  cr = gtk_snapshot_append_cairo (snapshot, &bounds);
  cairo_save (cr);

  width = gtk_widget_get_width (GTK_WIDGET (self));
  height = gtk_widget_get_height (GTK_WIDGET (self));
  // FIXME Port to GTK 4
  // gtk_render_background (context, cr, 0, 0, width, height);

  if (self->pixbuf && self->use_background) {
    gdk_cairo_set_source_pixbuf (cr, self->pixbuf, x, y);
    cairo_paint (cr);
  }

  cairo_restore (cr);
  cairo_destroy (cr);
}


static void
on_theme_name_changed (PhoshLockscreenBg *self, GParamSpec *pspec, PhoshStyleManager *style_manager)
{
  g_assert (PHOSH_IS_LOCKSCREEN_BG (self));
  g_assert (PHOSH_IS_STYLE_MANAGER (style_manager));

  self->use_background = !phosh_style_manager_is_high_contrast (style_manager);
  gtk_widget_queue_draw (GTK_WIDGET (self));
}


static void
phosh_lockscreen_bg_finalize (GObject *object)
{
  PhoshLockscreenBg *self = PHOSH_LOCKSCREEN_BG (object);

  g_clear_object (&self->bg_image);
  g_clear_object (&self->pixbuf);

  G_OBJECT_CLASS (phosh_lockscreen_bg_parent_class)->finalize (object);
}


static void
phosh_lockscreen_bg_class_init (PhoshLockscreenBgClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);
  PhoshLayerSurfaceClass *layer_surface_class = PHOSH_LAYER_SURFACE_CLASS (klass);

  object_class->finalize = phosh_lockscreen_bg_finalize;

  widget_class->snapshot = phosh_lockscreen_bg_snapshot;

  layer_surface_class->configured = phosh_lockscreen_bg_configured;

  gtk_widget_class_set_css_name (widget_class, "phosh-lockscreen-bg");
}


static void
phosh_lockscreen_bg_init (PhoshLockscreenBg *self)
{
  PhoshStyleManager *style_manager;

  style_manager = phosh_shell_get_style_manager (phosh_shell_get_default ());
  self->use_background = !phosh_style_manager_is_high_contrast (style_manager);
  g_signal_connect_object (style_manager,
                           "notify::theme-name",
                           G_CALLBACK (on_theme_name_changed),
                           self,
                           G_CONNECT_SWAPPED);
}


PhoshLockscreenBg *
phosh_lockscreen_bg_new (struct wl_output *wl_output)
{
  return g_object_new (PHOSH_TYPE_LOCKSCREEN_BG,
                       "wl-output", wl_output,
                       "anchor", (ZWLR_LAYER_SURFACE_V1_ANCHOR_TOP |
                                  ZWLR_LAYER_SURFACE_V1_ANCHOR_BOTTOM |
                                  ZWLR_LAYER_SURFACE_V1_ANCHOR_LEFT |
                                  ZWLR_LAYER_SURFACE_V1_ANCHOR_RIGHT),
                       "layer", ZWLR_LAYER_SHELL_V1_LAYER_OVERLAY,
                       "kbd-interactivity", FALSE,
                       "exclusive-zone", -1,
                       "namespace", "phosh lockscreen background",
                       NULL);
}


void
phosh_lockscreen_bg_set_image (PhoshLockscreenBg *self, PhoshBackgroundImage *image)
{
  g_return_if_fail (PHOSH_IS_LOCKSCREEN_BG (self));
  g_return_if_fail (image == NULL || PHOSH_IS_BACKGROUND_IMAGE (image));

  if (self->bg_image == image)
    return;

  g_set_object (&self->bg_image, image);
  update_image (self);
}
