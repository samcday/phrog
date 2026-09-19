/*
 * Copyright (C) 2024 Tether Operations Limited
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Arun Mani J <arun.mani@tether.to>
 */

#include "quick-settings-box.h"
#include "quick-setting.h"


static guint
count_children (PhoshQuickSettingsBox *box)
{
  int count = 0;
  GtkWidget *child = gtk_widget_get_first_child (GTK_WIDGET (box));

  g_return_val_if_fail (GTK_IS_REVEALER (child), 0);

  child = gtk_widget_get_next_sibling (child);
  while (child != NULL) {
    count += 1;
    child = gtk_widget_get_next_sibling (child);
  }

  return count;
}


static void
test_phosh_quick_settings_box_new (void)
{
  PhoshQuickSettingsBox *box;
  int max_columns;
  int spacing;
  gboolean can_show_status;
  guint count;
  g_autoptr (GList) children = NULL;

  box = g_object_new (PHOSH_TYPE_QUICK_SETTINGS_BOX, NULL);
  g_object_ref_sink (box);
  g_assert_true (PHOSH_IS_QUICK_SETTINGS_BOX (box));

  count = count_children (box);
  g_assert_cmpuint (count, ==, 0);

  max_columns = phosh_quick_settings_box_get_max_columns (box);
  g_assert_cmpuint (max_columns, ==, 3);

  spacing = phosh_quick_settings_box_get_spacing (box);
  g_assert_cmpuint (spacing, ==, 0);

  can_show_status = phosh_quick_settings_box_get_can_show_status (box);
  g_assert_true (can_show_status);

  g_assert_finalize_object (box);

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);
  g_assert_true (PHOSH_IS_QUICK_SETTINGS_BOX (box));
  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_add (void)
{
  GtkWidget *box;
  GtkWidget *child;
  guint count;

  box = phosh_quick_settings_box_new (3, 0);
  g_object_ref_sink (box);
  child = phosh_quick_setting_new (NULL);

  phosh_quick_settings_box_add (PHOSH_QUICK_SETTINGS_BOX (box), PHOSH_QUICK_SETTING (child));
  count = count_children (PHOSH_QUICK_SETTINGS_BOX (box));
  g_assert_cmpuint (count, ==, 1);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_remove (void)
{
  GtkWidget *box;
  GtkWidget *child;
  guint count;


  box = phosh_quick_settings_box_new (3, 0);
  g_object_ref_sink (box);
  child = phosh_quick_setting_new (NULL);

  phosh_quick_settings_box_add (PHOSH_QUICK_SETTINGS_BOX (box), PHOSH_QUICK_SETTING (child));
  phosh_quick_settings_box_remove (PHOSH_QUICK_SETTINGS_BOX (box), PHOSH_QUICK_SETTING (child));
  count = count_children (PHOSH_QUICK_SETTINGS_BOX (box));
  g_assert_cmpuint (count, ==, 0);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_get_max_columns (void)
{
  PhoshQuickSettingsBox *box;
  guint max_columns;
  guint got_max_columns;

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);

  max_columns = 6;
  phosh_quick_settings_box_set_max_columns (box, max_columns);
  got_max_columns = phosh_quick_settings_box_get_max_columns (box);
  g_assert_cmpuint (got_max_columns, ==, max_columns);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_get_spacing (void)
{
  PhoshQuickSettingsBox *box;
  guint spacing;
  guint got_spacing;

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);

  spacing = 6;
  phosh_quick_settings_box_set_spacing (box, spacing);
  got_spacing = phosh_quick_settings_box_get_spacing (box);
  g_assert_cmpuint (got_spacing, ==, spacing);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_set_can_show_status (void)
{
  PhoshQuickSettingsBox *box;
  PhoshQuickSetting *child_a;
  PhoshQuickSetting *child_b;
  gboolean can_show_status;
  gboolean got_can_show_status;

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);
  child_a = g_object_new (PHOSH_TYPE_QUICK_SETTING, "can-show-status", TRUE, NULL);
  phosh_quick_settings_box_add (box, child_a);
  child_b = g_object_new (PHOSH_TYPE_QUICK_SETTING, "can-show-status", FALSE, NULL);
  phosh_quick_settings_box_add (box, child_b);

  can_show_status = TRUE;
  phosh_quick_settings_box_set_can_show_status (box, can_show_status);
  got_can_show_status = phosh_quick_setting_get_can_show_status (child_a);
  g_assert_true (got_can_show_status == can_show_status);
  got_can_show_status = phosh_quick_setting_get_can_show_status (child_b);
  g_assert_true (got_can_show_status == can_show_status);

  can_show_status = FALSE;
  phosh_quick_settings_box_set_can_show_status (box, can_show_status);
  got_can_show_status = phosh_quick_setting_get_can_show_status (child_a);
  g_assert_true (got_can_show_status == can_show_status);
  got_can_show_status = phosh_quick_setting_get_can_show_status (child_b);
  g_assert_true (got_can_show_status == can_show_status);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_get_can_show_status (void)
{
  PhoshQuickSettingsBox *box;
  gboolean can_show_status;
  gboolean got_can_show_status;

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);

  can_show_status = FALSE;
  phosh_quick_settings_box_set_can_show_status (box, can_show_status);
  got_can_show_status = phosh_quick_settings_box_get_can_show_status (box);
  g_assert_true (got_can_show_status == can_show_status);

  g_assert_finalize_object (box);
}


static void
test_phosh_quick_settings_box_hide_status (void)
{
  PhoshQuickSettingsBox *box;
  PhoshQuickSetting *child;
  gboolean showing_status;
  gboolean got_showing_status;

  box = PHOSH_QUICK_SETTINGS_BOX (phosh_quick_settings_box_new (3, 0));
  g_object_ref_sink (box);

  child = PHOSH_QUICK_SETTING (phosh_quick_setting_new (phosh_status_page_new ()));
  phosh_quick_settings_box_add (box, child);

  g_signal_emit_by_name (child, "show-status");
  showing_status = TRUE;
  got_showing_status = phosh_quick_setting_get_showing_status (child);
  g_assert_true (got_showing_status == showing_status);

  phosh_quick_settings_box_hide_status (box);
  showing_status = FALSE;
  got_showing_status = phosh_quick_setting_get_showing_status (child);
  g_assert_true (got_showing_status == showing_status);

  g_assert_finalize_object (box);
}


int
main (int argc, char *argv[])
{
  gtk_test_init (&argc, &argv, NULL);

  g_test_add_func ("/phosh/quick-settings-box/new",
                   test_phosh_quick_settings_box_new);
  g_test_add_func ("/phosh/quick-settings-box/add",
                   test_phosh_quick_settings_box_add);
  g_test_add_func ("/phosh/quick-settings-box/remove",
                   test_phosh_quick_settings_box_remove);
  g_test_add_func ("/phosh/quick-settings-box/get_max_columns",
                   test_phosh_quick_settings_box_get_max_columns);
  g_test_add_func ("/phosh/quick-settings-box/get_spacing",
                   test_phosh_quick_settings_box_get_spacing);
  g_test_add_func ("/phosh/quick-settings-box/set_can_show_status",
                   test_phosh_quick_settings_box_set_can_show_status);
  g_test_add_func ("/phosh/quick-settings-box/get_can_show_status",
                   test_phosh_quick_settings_box_get_can_show_status);
  g_test_add_func ("/phosh/quick-settings-box/hide_status",
                   test_phosh_quick_settings_box_hide_status);

  return g_test_run ();
}
