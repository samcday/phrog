/*
 * Copyright (C) 2020 Purism SPC
 *               2024 Tether Operations Limited
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Authors: Guido Günther <agx@sigxcpu.org>
 *          Arun Mani J <arun.mani@tether.to>
 */

#include "quick-setting.h"
#include "status-icon.h"


static void
test_phosh_quick_setting_new (void)
{
  PhoshQuickSetting *quick_setting;
  gboolean active;
  gboolean show_status;
  gboolean can_show_status;
  PhoshStatusPage *status_page;
  const char *action_name;
  const char *action_target;

  quick_setting = g_object_ref_sink (g_object_new (PHOSH_TYPE_QUICK_SETTING, NULL));
  g_assert_true (PHOSH_IS_QUICK_SETTING (quick_setting));

  active = phosh_quick_setting_get_active (quick_setting);
  g_assert_false (active);

  show_status = phosh_quick_setting_get_showing_status (quick_setting);
  g_assert_false (show_status);

  can_show_status = phosh_quick_setting_get_can_show_status (quick_setting);
  g_assert_false (can_show_status);

  status_page = phosh_quick_setting_get_status_page (quick_setting);
  g_assert_true (status_page == NULL);

  action_name = phosh_quick_setting_get_long_press_action_name (quick_setting);
  g_assert_true (action_name == NULL);

  action_target = phosh_quick_setting_get_long_press_action_target (quick_setting);
  g_assert_true (action_target == NULL);

  g_object_unref (GTK_WIDGET (quick_setting));

  status_page = phosh_status_page_new ();
  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (status_page)));
  g_assert_true (PHOSH_IS_QUICK_SETTING (quick_setting));
  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_add_status_icon (void)
{
  GtkWidget *quick_setting;
  PhoshStatusIcon *status_icon;
  GtkWidget *button;
  GtkWidget *button_box;
  PhoshStatusIcon *icon_wid;
  GtkLabel *label_wid;
  const char *label;
  const char *got_label;

  quick_setting = g_object_ref_sink (phosh_quick_setting_new (NULL));

  label = "Foo";
  status_icon = g_object_new (PHOSH_TYPE_STATUS_ICON, "icon-name", "face-smile-symbolic", "info",
                              label, NULL);
  phosh_quick_setting_set_status_icon (PHOSH_QUICK_SETTING (quick_setting), status_icon);

  button = gtk_widget_get_first_child (quick_setting);
  button_box = gtk_button_get_child (GTK_BUTTON (button));
  icon_wid = PHOSH_STATUS_ICON (gtk_widget_get_first_child (button_box));
  label_wid = GTK_LABEL (gtk_widget_get_last_child (button_box));

  g_assert_true (icon_wid == status_icon);

  got_label = gtk_label_get_text (label_wid);
  g_assert_cmpstr (label, ==, got_label);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_remove_status_icon (void)
{
  GtkWidget *quick_setting;
  PhoshStatusIcon *status_icon;
  GtkWidget *button;
  GtkWidget *button_box;
  GtkWidget *icon_wid;
  GtkWidget *label_wid;


  quick_setting = g_object_ref_sink (phosh_quick_setting_new (NULL));

  status_icon = PHOSH_STATUS_ICON (phosh_status_icon_new ());
  phosh_quick_setting_set_status_icon (PHOSH_QUICK_SETTING (quick_setting), status_icon);
  phosh_quick_setting_set_status_icon (PHOSH_QUICK_SETTING (quick_setting), NULL);

  button = gtk_widget_get_first_child (quick_setting);
  button_box = gtk_button_get_child (GTK_BUTTON (button));
  icon_wid = gtk_widget_get_first_child (button_box);
  label_wid = gtk_widget_get_last_child (button_box);

  g_assert_true (icon_wid == label_wid);
  g_assert_false (PHOSH_IS_STATUS_ICON (icon_wid));

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_set_active (void)
{
  GtkWidget *quick_setting;
  GtkStateFlags flags;

  quick_setting = g_object_ref_sink (phosh_quick_setting_new (NULL));

  phosh_quick_setting_set_active (PHOSH_QUICK_SETTING (quick_setting), TRUE);
  flags = gtk_widget_get_state_flags (quick_setting);
  g_assert_true (flags & GTK_STATE_FLAG_CHECKED);

  phosh_quick_setting_set_active (PHOSH_QUICK_SETTING (quick_setting), FALSE);
  flags = gtk_widget_get_state_flags (quick_setting);
  g_assert_false (flags & GTK_STATE_FLAG_CHECKED);

  g_object_unref (quick_setting);
}


static void
test_phosh_quick_setting_get_active (void)
{
  PhoshQuickSetting *quick_setting;
  gboolean active;
  gboolean got_active;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  active = TRUE;
  phosh_quick_setting_set_active (quick_setting, active);
  got_active = phosh_quick_setting_get_active (quick_setting);
  g_assert_true (got_active == active);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_set_showing_status (void)
{
  PhoshQuickSetting *quick_setting;
  GtkWidget *arrow_btn;
  GtkWidget *arrow;
  const char *icon_name;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  arrow_btn = gtk_widget_get_last_child (GTK_WIDGET (quick_setting));
  arrow = gtk_button_get_child (GTK_BUTTON (arrow_btn));

  phosh_quick_setting_set_showing_status (quick_setting, TRUE);
  icon_name = gtk_image_get_icon_name (GTK_IMAGE (arrow));
  g_assert_cmpstr ("go-down-symbolic", ==, icon_name);

  phosh_quick_setting_set_showing_status (quick_setting, FALSE);
  icon_name = gtk_image_get_icon_name (GTK_IMAGE (arrow));
  g_assert_cmpstr ("go-next-symbolic", ==, icon_name);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_get_showing_status (void)
{
  PhoshQuickSetting *quick_setting;
  gboolean showing_status;
  gboolean got_showing_status;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  showing_status = TRUE;
  phosh_quick_setting_set_showing_status (quick_setting, showing_status);
  got_showing_status = phosh_quick_setting_get_showing_status (quick_setting);
  g_assert_true (got_showing_status == showing_status);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_set_can_show_status (void)
{
  PhoshQuickSetting *quick_setting;
  GtkWidget *arrow_btn;
  gboolean can_show_status;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (phosh_status_page_new ())));

  arrow_btn = gtk_widget_get_last_child (GTK_WIDGET (quick_setting));

  can_show_status = TRUE;
  phosh_quick_setting_set_can_show_status (quick_setting, can_show_status);
  g_assert_true (gtk_widget_get_visible (arrow_btn));

  can_show_status = FALSE;
  phosh_quick_setting_set_can_show_status (quick_setting, can_show_status);
  g_assert_false (gtk_widget_get_visible (arrow_btn));

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_get_can_show_status (void)
{
  PhoshQuickSetting *quick_setting;
  gboolean can_show_status;
  gboolean got_can_show_status;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  can_show_status = TRUE;
  phosh_quick_setting_set_can_show_status (quick_setting, can_show_status);
  got_can_show_status = phosh_quick_setting_get_can_show_status (quick_setting);
  g_assert_true (got_can_show_status == can_show_status);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_set_status_page (void)
{
  PhoshQuickSetting *quick_setting;
  PhoshStatusPage *status_page;
  GtkWidget *arrow_btn;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));
  phosh_quick_setting_set_can_show_status (quick_setting, TRUE);

  arrow_btn = gtk_widget_get_last_child (GTK_WIDGET (quick_setting));

  g_assert_false (gtk_widget_get_visible (arrow_btn));

  status_page = phosh_status_page_new ();
  phosh_quick_setting_set_status_page (quick_setting, status_page);
  g_assert_true (gtk_widget_get_visible (arrow_btn));

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_get_status_page (void)
{
  PhoshQuickSetting *quick_setting;
  PhoshStatusPage *status_page;
  PhoshStatusPage *got_status_page;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  status_page = phosh_status_page_new ();
  phosh_quick_setting_set_status_page (quick_setting, status_page);
  got_status_page = phosh_quick_setting_get_status_page (quick_setting);
  g_assert_true (got_status_page == status_page);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_get_long_press_action_name (void)
{
  PhoshQuickSetting *quick_setting;
  const char *action_name;
  const char *got_action_name;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  action_name = "foo";
  phosh_quick_setting_set_long_press_action_name (quick_setting, action_name);
  got_action_name = phosh_quick_setting_get_long_press_action_name (quick_setting);
  g_assert_cmpstr (action_name, ==, got_action_name);

  g_object_unref (GTK_WIDGET (quick_setting));
}


static void
test_phosh_quick_setting_get_long_press_action_target (void)
{
  PhoshQuickSetting *quick_setting;
  const char *action_target;
  const char *got_action_target;

  quick_setting = g_object_ref_sink (PHOSH_QUICK_SETTING (phosh_quick_setting_new (NULL)));

  action_target = "foo";
  phosh_quick_setting_set_long_press_action_target (quick_setting, action_target);
  got_action_target = phosh_quick_setting_get_long_press_action_target (quick_setting);
  g_assert_cmpstr (action_target, ==, got_action_target);

  g_object_unref (GTK_WIDGET (quick_setting));
}


int
main (int argc, char *argv[])
{
  gtk_test_init (&argc, &argv, NULL);

  g_test_add_func ("/phosh/quick-setting/new",
                   test_phosh_quick_setting_new);
  g_test_add_func ("/phosh/quick-setting/add_status_icon",
                   test_phosh_quick_setting_add_status_icon);
  g_test_add_func ("/phosh/quick-setting/remove_status_icon",
                   test_phosh_quick_setting_remove_status_icon);
  g_test_add_func ("/phosh/quick-setting/set_active",
                   test_phosh_quick_setting_set_active);
  g_test_add_func ("/phosh/quick-setting/get_active",
                   test_phosh_quick_setting_get_active);
  g_test_add_func ("/phosh/quick-setting/set_can_show_status",
                   test_phosh_quick_setting_set_can_show_status);
  g_test_add_func ("/phosh/quick-setting/get_can_show_status",
                   test_phosh_quick_setting_get_can_show_status);
  g_test_add_func ("/phosh/quick-setting/set_showing_status",
                   test_phosh_quick_setting_set_showing_status);
  g_test_add_func ("/phosh/quick-setting/get_showing_status",
                   test_phosh_quick_setting_get_showing_status);
  g_test_add_func ("/phosh/quick-setting/set_status_page",
                   test_phosh_quick_setting_set_status_page);
  g_test_add_func ("/phosh/quick-setting/get_status_page",
                   test_phosh_quick_setting_get_status_page);
  g_test_add_func ("/phosh/quick-setting/get_long_press_action_name",
                   test_phosh_quick_setting_get_long_press_action_name);
  g_test_add_func ("/phosh/quick-setting/get_long_press_action_target",
                   test_phosh_quick_setting_get_long_press_action_target);

  return g_test_run ();
}
