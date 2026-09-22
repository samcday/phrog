/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "calendar.h"

/**
 * PhoshCalendar
 *
 * Display the calenadar.
 */
struct _PhoshCalendar {
  GtkBox       parent;

  GtkCalendar *calendar;
  GSettings   *calendar_settings;
};

G_DEFINE_TYPE (PhoshCalendar, phosh_calendar, GTK_TYPE_BOX);


static void
phosh_calendar_dispose (GObject *object)
{
  PhoshCalendar *self = PHOSH_CALENDAR (object);

  g_clear_object (&self->calendar_settings);

  G_OBJECT_CLASS (phosh_calendar_parent_class)->dispose (object);
}


static void
on_settings_changed (GSettings  *settings,
                     const char *key,
                     gpointer    user_data)
{
  PhoshCalendar *self = PHOSH_CALENDAR (user_data);
  gboolean display_week;

  display_week = g_settings_get_boolean (settings, key);
  gtk_calendar_set_show_week_numbers (self->calendar, display_week);
}


static void
phosh_calendar_class_init (PhoshCalendarClass *klass)
{
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);
  GObjectClass *object_class = G_OBJECT_CLASS (klass);

  object_class->dispose = phosh_calendar_dispose;

  g_type_ensure (GTK_TYPE_CALENDAR);

  gtk_widget_class_set_template_from_resource (widget_class,
                                               "/mobi/phosh/plugins/calendar/calendar.ui");
  gtk_widget_class_bind_template_child (widget_class, PhoshCalendar, calendar);

  gtk_widget_class_set_css_name (widget_class, "phosh-calendar");
}

static void
phosh_calendar_init (PhoshCalendar *self)
{
  g_autoptr (GtkCssProvider) css_provider = NULL;

  gtk_widget_init_template (GTK_WIDGET (self));

  css_provider = gtk_css_provider_new ();
  gtk_css_provider_load_from_resource (css_provider,
                                       "/mobi/phosh/plugins/calendar/stylesheet/common.css");
  gtk_style_context_add_provider_for_display (gdk_display_get_default (),
                                              GTK_STYLE_PROVIDER (css_provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

  self->calendar_settings = g_settings_new ("org.gnome.desktop.calendar");
  g_signal_connect (self->calendar_settings,
                    "changed::show-weekdate",
                    G_CALLBACK (on_settings_changed),
                    self);
  on_settings_changed (self->calendar_settings, "show-weekdate", self);
}
