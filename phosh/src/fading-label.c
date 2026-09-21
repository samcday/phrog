/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Alexander Mikhaylenko <alexander.mikhaylenko@puri.sm>
 *
 * Based on <hdy-fading-label.c> from <libhandy 1.5.0> which is LGPL-2.1+.
 */

#include "phosh-config.h"
#include "fading-label.h"

#include <glib/gi18n-lib.h>
#include "bidi.h"

/**
 * PhoshFadingLabel:
 *
 * A label that visually fades out when too wide for the given space.
 */

#define FADE_WIDTH 18

struct _PhoshFadingLabel
{
  GtkWidget parent_instance;

  GtkWidget *label;
  gfloat align;
  cairo_pattern_t *gradient;
};

G_DEFINE_TYPE (PhoshFadingLabel, phosh_fading_label, GTK_TYPE_WIDGET)

enum {
  PROP_0,
  PROP_LABEL,
  PROP_ALIGN,
  LAST_PROP
};

static GParamSpec *props[LAST_PROP];

static gboolean
is_rtl (PhoshFadingLabel *self)
{
  PangoDirection pango_direction = PANGO_DIRECTION_NEUTRAL;
  const char *label = phosh_fading_label_get_label (self);

  if (label)
    pango_direction = phosh_find_base_dir (label, -1);

  if (pango_direction == PANGO_DIRECTION_RTL)
    return TRUE;

  if (pango_direction == PANGO_DIRECTION_LTR)
    return FALSE;

  return gtk_widget_get_direction (GTK_WIDGET (self)) == GTK_TEXT_DIR_RTL;
}

static void
ensure_gradient (PhoshFadingLabel *self)
{
  if (self->gradient)
    return;

  self->gradient = cairo_pattern_create_linear (0, 0, 1, 0);
  cairo_pattern_add_color_stop_rgba (self->gradient, 0, 1, 1, 1, 0);
  cairo_pattern_add_color_stop_rgba (self->gradient, 1, 1, 1, 1, 1);
}

static void
phosh_fading_label_measure (GtkWidget     *widget,
                            GtkOrientation orientation,
                            int            for_size,
                            int           *minimum,
                            int           *natural,
                            int           *minimum_baseline,
                            int           *natural_baseline)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (widget);

  gtk_widget_measure (self->label,
                      orientation,
                      for_size,
                      minimum, natural,
                      minimum_baseline, natural_baseline);

  if (orientation == GTK_ORIENTATION_HORIZONTAL) {
    if (minimum)
      *minimum = 0;
  }
}

static void
phosh_fading_label_size_allocate (GtkWidget *widget,
                                  int        width,
                                  int        height,
                                  int        baseline)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (widget);
  gfloat align = is_rtl (self) ? 1 - self->align : self->align;
  GtkAllocation child_allocation;
  gint child_width;

  phosh_fading_label_measure (widget,
                              GTK_ORIENTATION_HORIZONTAL,
                              -1,
                              NULL, &child_width,
                              NULL, NULL);

  child_allocation.x = (gint) ((width - child_width) * align);
  child_allocation.y = 0;
  child_allocation.width = child_width;
  child_allocation.height = height;

  gtk_widget_size_allocate (self->label, &child_allocation, baseline);
}

static void
phosh_fading_label_snapshot (GtkWidget   *widget,
                             GtkSnapshot *snapshot)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (widget);
  cairo_t *cr;
  gfloat align = is_rtl (self) ? 1 - self->align : self->align;
  graphene_rect_t clip, alloc;
  int child_width = gtk_widget_get_width (self->label);

  g_return_if_fail (gtk_widget_compute_bounds (widget, widget, &alloc));

  if (child_width <= alloc.size.width) {
    gtk_widget_snapshot_child (widget, self->label, snapshot);
    return;
  }

  ensure_gradient (self);

  g_return_if_fail (gtk_widget_compute_bounds (widget, widget, &clip));
  clip.origin.x = 0;
  clip.origin.y -= alloc.origin.y;
  clip.size.width = alloc.size.width;

  gtk_snapshot_push_clip (snapshot, &alloc);
  cr = gtk_snapshot_append_cairo (snapshot, &alloc);

  cairo_save (cr);
  cairo_rectangle (cr, clip.origin.x, clip.origin.y, clip.size.width, clip.size.height);
  cairo_clip (cr);

  cairo_push_group (cr);
  gtk_widget_snapshot_child (widget, self->label, snapshot);

  if (align > 0) {
      cairo_save (cr);
      cairo_translate (cr, clip.origin.x + FADE_WIDTH, clip.origin.y);
      cairo_scale (cr, -FADE_WIDTH, clip.size.height);
      cairo_set_source (cr, self->gradient);
      cairo_rectangle (cr, 0, 0, 1, 1);
      cairo_set_operator (cr, CAIRO_OPERATOR_DEST_OUT);
      cairo_fill (cr);
      cairo_restore (cr);
  }

  if (align < 1) {
      cairo_translate (cr, clip.origin.x + clip.size.width - FADE_WIDTH, clip.origin.y);
      cairo_scale (cr, FADE_WIDTH, clip.size.height);
      cairo_set_source (cr, self->gradient);
      cairo_rectangle (cr, 0, 0, 1, 1);
      cairo_set_operator (cr, CAIRO_OPERATOR_DEST_OUT);
      cairo_fill (cr);
  }

  cairo_pop_group_to_source (cr);
  cairo_paint (cr);

  cairo_restore (cr);
  cairo_destroy (cr);
  gtk_snapshot_pop (snapshot);
}

static void
phosh_fading_label_get_property (GObject    *object,
                                 guint       prop_id,
                                 GValue     *value,
                                 GParamSpec *pspec)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (object);

  switch (prop_id) {
  case PROP_LABEL:
    g_value_set_string (value, phosh_fading_label_get_label (self));
    break;

  case PROP_ALIGN:
    g_value_set_float (value, phosh_fading_label_get_align (self));
    break;

    default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, prop_id, pspec);
  }
}

static void
phosh_fading_label_set_property (GObject      *object,
                                 guint         prop_id,
                                 const GValue *value,
                                 GParamSpec   *pspec)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (object);

  switch (prop_id) {
  case PROP_LABEL:
    phosh_fading_label_set_label (self, g_value_get_string (value));
    break;

  case PROP_ALIGN:
    phosh_fading_label_set_align (self, g_value_get_float (value));
    break;

  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, prop_id, pspec);
  }
}

static void
phosh_fading_label_dispose (GObject *object)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (object);

  g_clear_pointer (&self->label, gtk_widget_unparent);

  G_OBJECT_CLASS (phosh_fading_label_parent_class)->dispose (object);
}

static void
phosh_fading_label_finalize (GObject *object)
{
  PhoshFadingLabel *self = PHOSH_FADING_LABEL (object);

  g_clear_pointer (&self->gradient, cairo_pattern_destroy);

  G_OBJECT_CLASS (phosh_fading_label_parent_class)->finalize (object);
}

static void
phosh_fading_label_class_init (PhoshFadingLabelClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->get_property = phosh_fading_label_get_property;
  object_class->set_property = phosh_fading_label_set_property;
  object_class->dispose = phosh_fading_label_dispose;
  object_class->finalize = phosh_fading_label_finalize;

  widget_class->measure = phosh_fading_label_measure;
  widget_class->size_allocate = phosh_fading_label_size_allocate;
  widget_class->snapshot = phosh_fading_label_snapshot;

  props[PROP_LABEL] =
    g_param_spec_string ("label", "", "",
                         NULL,
                         G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  props[PROP_ALIGN] =
    g_param_spec_float ("align", "", "",
                        0.0, 1.0, 0.0,
                        G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  g_object_class_install_properties (object_class, LAST_PROP, props);
}

static void
phosh_fading_label_init (PhoshFadingLabel *self)
{
  self->label = gtk_label_new (NULL);
  gtk_widget_set_visible (self->label, TRUE);
  gtk_label_set_single_line_mode (GTK_LABEL (self->label), TRUE);
  gtk_widget_set_parent (self->label, GTK_WIDGET (self));
}

GtkWidget *
phosh_fading_label_new (const char *label)
{
  return GTK_WIDGET (g_object_new (PHOSH_TYPE_FADING_LABEL, "label", label, NULL));
}

const char *
phosh_fading_label_get_label (PhoshFadingLabel *self)
{
  g_return_val_if_fail (PHOSH_IS_FADING_LABEL (self), NULL);

  return gtk_label_get_label (GTK_LABEL (self->label));
}

void
phosh_fading_label_set_label (PhoshFadingLabel *self,
                              const char       *label)
{
  g_return_if_fail (PHOSH_IS_FADING_LABEL (self));

  if (!g_strcmp0 (label, phosh_fading_label_get_label (self)))
    return;

  gtk_label_set_label (GTK_LABEL (self->label), label);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_LABEL]);
}

float
phosh_fading_label_get_align (PhoshFadingLabel *self)
{
  g_return_val_if_fail (PHOSH_IS_FADING_LABEL (self), 0.0f);

  return self->align;
}

void
phosh_fading_label_set_align (PhoshFadingLabel *self,
                              gfloat            align)
{
  g_return_if_fail (PHOSH_IS_FADING_LABEL (self));

  align = CLAMP (align, 0.0, 1.0);

  if (!(self->align > align || self->align < align))
    return;

  self->align = align;

  gtk_widget_queue_allocate (GTK_WIDGET (self));

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_ALIGN]);
}
