/*
 * Copyright © 2020 Zander Brown <zbrown@gnome.org>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Zander Brown <zbrown@gnome.org>
 *
 * A "real" notification daemon for testing the notification list
 *
 * NOTE: Remember to close this, otherwise you'll miss things
 *
 * If you just want to play around with styles, see notify-blocks
 */

#include <adwaita.h>
#include <notifications/notify-manager.h>
#include <notifications/notification-frame.h>


static GtkWidget *
create (gpointer item, gpointer data)
{
  GtkWidget *row = NULL;
  GtkWidget *frame = NULL;

  row = g_object_new (GTK_TYPE_LIST_BOX_ROW,
                      "activatable", FALSE,
                      NULL);

  frame = phosh_notification_frame_new (TRUE, NULL);
  phosh_notification_frame_bind_model (PHOSH_NOTIFICATION_FRAME (frame), item);

  gtk_widget_set_visible (frame, TRUE);

  gtk_list_box_row_set_child (GTK_LIST_BOX_ROW (row), frame);

  return row;
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
  GtkListBox *box;
  PhoshNotifyManager *manager;

  css_setup ();

  window = g_object_new (GTK_TYPE_APPLICATION_WINDOW,
                         "application", app,
                         "default-height", 640,
                         "default-width", 360,
                         "height-request", 640,
                         "width-request", 360,
                         "title", "PhoshNotification Demo",
                         NULL);

  scrolled = g_object_new (GTK_TYPE_SCROLLED_WINDOW,
                           "vscrollbar-policy", GTK_POLICY_AUTOMATIC,
                           "hscrollbar-policy", GTK_POLICY_NEVER,
                           NULL);
  gtk_window_set_child (window, GTK_WIDGET (scrolled));

  box = g_object_new (GTK_TYPE_LIST_BOX,
                      "selection-mode", GTK_SELECTION_NONE,
                      NULL);
  gtk_scrolled_window_set_child (scrolled, GTK_WIDGET (box));

  manager = phosh_notify_manager_get_default ();
  gtk_list_box_bind_model (box,
                           G_LIST_MODEL (phosh_notify_manager_get_list (manager)),
                           create,
                           NULL,
                           NULL);

  gtk_window_present (window);
}


int
main (int argc, char **argv)
{
  g_autoptr (AdwApplication) app = NULL;

  app = adw_application_new ("mobi.phosh.tools.NotifyServerStandalone",
                             G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect (app, "activate", G_CALLBACK (on_activate), NULL);
  return g_application_run (G_APPLICATION (app), argc, argv);
}
