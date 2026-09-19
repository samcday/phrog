/*
 * Copyright © 2020 Zander Brown <zbrown@gnome.org>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Zander Brown <zbrown@gnome.org>
 *
 * Static notification blocks for experimenting with styles
 *
 * For testing the notification server (rather than widget style) see
 * notify-server-standalone
 */

#include <adwaita.h>
#include <gio/gdesktopappinfo.h>
#include <notifications/notification-frame.h>
#include <notifications/notification.h>


static void
empty (PhoshNotificationFrame *self, GtkBox *box)
{
  gtk_box_remove (box, GTK_WIDGET (self));
}


static void
css_setup (void)
{
  GtkCssProvider *provider = NULL;
  GFile *file = NULL;

  provider = gtk_css_provider_new ();
  file = g_file_new_for_uri ("resource:///mobi/phosh/stylesheet/adwaita-dark.css");
  gtk_css_provider_load_from_file (provider, file);
  gtk_style_context_add_provider_for_display (gdk_display_get_default (),
                                              GTK_STYLE_PROVIDER (provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
  g_object_unref (file);

  g_object_set (adw_style_manager_get_default (),
                "color-scheme", ADW_COLOR_SCHEME_FORCE_DARK,
                NULL);
}


static void
on_activate (AdwApplication *app)
{
  GtkWindow *window;
  GtkScrolledWindow *scrolled;
  GtkBox *box;
  GtkWidget *frame;
  GDesktopAppInfo *info;
  GStrv actions = (char *[]) { "ok", "Okay", NULL };
  GIcon *image = NULL;
  PhoshNotification *notification = NULL;
  g_autoptr (GDateTime) now = g_date_time_new_now_local ();

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "default-height", 640,
                         "default-width", 360,
                         "height-request", 640,
                         "width-request", 360,
                         "title", "PhoshNotificationFrame Demo",
                         NULL);

  scrolled = g_object_new (GTK_TYPE_SCROLLED_WINDOW,
                           "vscrollbar-policy", GTK_POLICY_NEVER,
                           NULL);
  gtk_window_set_child (window, GTK_WIDGET (scrolled));

  box = g_object_new (GTK_TYPE_BOX,
                      "margin-start", 6,
                      "margin-top", 6,
                      "margin-end", 6,
                      "margin-bottom", 6,
                      "orientation", GTK_ORIENTATION_VERTICAL,
                      NULL);
  gtk_scrolled_window_set_child (scrolled, GTK_WIDGET (box));

  info = g_desktop_app_info_new ("org.gnome.Calculator.desktop");
  notification = phosh_notification_new (0,
                                         "Not Shown",
                                         G_APP_INFO (info),
                                         "2 + 2",
                                         "= 4",
                                         NULL,
                                         NULL,
                                         PHOSH_NOTIFICATION_URGENCY_NORMAL,
                                         actions,
                                         FALSE,
                                         FALSE,
                                         NULL,
                                         NULL,
                                         now);
  frame = phosh_notification_frame_new (TRUE, NULL);
  phosh_notification_frame_bind_notification (PHOSH_NOTIFICATION_FRAME (frame),
                                              notification);
  g_signal_connect (frame, "empty", G_CALLBACK (empty), box);
  gtk_box_append (box, frame);

  image = g_themed_icon_new ("org.gnome.Software");
  notification = phosh_notification_new (1,
                                         "Some App",
                                         NULL,
                                         "2 + 2",
                                         "= 4",
                                         NULL,
                                         image,
                                         PHOSH_NOTIFICATION_URGENCY_NORMAL,
                                         NULL,
                                         FALSE,
                                         FALSE,
                                         NULL,
                                         NULL,
                                         now);
  frame = phosh_notification_frame_new (TRUE, NULL);
  phosh_notification_frame_bind_notification (PHOSH_NOTIFICATION_FRAME (frame),
                                              notification);
  g_signal_connect (frame, "empty", G_CALLBACK (empty), box);
  gtk_box_append (box, frame);

  gtk_window_present (window);
}


int
main (int argc, char **argv)
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.NotificationFrameDemo",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
