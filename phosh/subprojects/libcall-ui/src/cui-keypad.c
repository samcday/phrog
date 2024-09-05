/*
 * Copyright (C) 2021 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * based HdyKeypad which is
 * Copyright (C) 2019 Purism SPC
 */

#include "cui-config.h"

#include <glib.h>
#include <glib/gi18n-lib.h>

#include "cui-keypad.h"
#include "cui-keypad-button-private.h"

/**
 * CuiKeypad:
 *
 * A keypad for dialing numbers
 *
 * The `CuiKeypad` widget mimics a physical keypad
 * for entering numbers such as phone numbers or PIN codes.
 */


enum {
  PROP_0,
  PROP_ROW_SPACING,
  PROP_COLUMN_SPACING,
  PROP_LETTERS_VISIBLE,
  PROP_SYMBOLS_VISIBLE,
  PROP_ENTRY,
  PROP_END_ACTION,
  PROP_START_ACTION,
  PROP_LAST_PROP,
};
static GParamSpec *props[PROP_LAST_PROP];

struct _CuiKeypad {
  GtkBin      parent_instance;

  GtkEntry   *entry;
  GtkGrid    *grid;
  GtkLabel   *label_asterisk;
  GtkLabel   *label_hash;

  GtkGesture *long_press_zero_gesture;

  guint16     row_spacing;
  guint16     column_spacing;
  gboolean    symbols_visible;
  gboolean    letters_visible;

  GRegex     *re_separators;
  GRegex     *re_no_digits;
  GRegex     *re_no_digits_or_symbols;
};

G_DEFINE_TYPE (CuiKeypad, cui_keypad, GTK_TYPE_BIN)

static void
symbol_clicked (CuiKeypad *self,
                gchar      symbol)
{
  g_autofree gchar *string = g_strdup_printf ("%c", symbol);

  if (!self->entry)
    return;

  g_signal_emit_by_name (self->entry, "insert-at-cursor", string, NULL);
  /* Set focus to the entry only when it can get focus
   * https://gitlab.gnome.org/GNOME/gtk/issues/2204
   */
  if (gtk_widget_get_can_focus (GTK_WIDGET (self->entry)))
    gtk_entry_grab_focus_without_selecting (self->entry);
}


static void
button_clicked_cb (CuiKeypad       *self,
                   CuiKeypadButton *btn)
{
  gchar digit = cui_keypad_button_get_digit (btn);

  symbol_clicked (self, digit);
  g_debug ("Button with number %c was pressed", digit);
}


static void
asterisk_button_clicked_cb (CuiKeypad *self)
{
  symbol_clicked (self, '*');
  g_debug ("Button with * was pressed");
}


static void
hash_button_clicked_cb (CuiKeypad *self)
{
  symbol_clicked (self, '#');
  g_debug ("Button with # was pressed");
}


static void
insert_text_cb (CuiKeypad   *self,
                gchar       *text,
                gint         length,
                gpointer     position,
                GtkEditable *editable)
{
  g_autofree char *no_separators = NULL;
  g_autoptr (GError) error = NULL;

  g_assert (g_utf8_validate (text, length, NULL));

  /* Get rid of visual separators and potentially resubmit text insertion */
  no_separators = g_regex_replace_literal (self->re_separators, text, -1, 0, "", 0, &error);
  if (!no_separators) {
    g_warning ("Error replacing visual characters in '%s': %s",
               text, error->message);
    gtk_widget_error_bell (GTK_WIDGET (editable));
    g_signal_stop_emission_by_name (editable, "insert-text");
  } else if (g_strcmp0 (text, no_separators)) {
      g_signal_stop_emission_by_name (editable, "insert-text");
      gtk_editable_insert_text (editable, no_separators, -1, position);
      return;
  }

  /* Validate input */
  if ((self->symbols_visible &&
       g_regex_match (self->re_no_digits_or_symbols, text, 0, NULL)) ||
      (!self->symbols_visible &&
       g_regex_match (self->re_no_digits, text, 0, NULL))) {
    gtk_widget_error_bell (GTK_WIDGET (editable));
    g_signal_stop_emission_by_name (editable, "insert-text");
  }
}


static void
long_press_zero_cb (CuiKeypad  *self,
                    gdouble     x,
                    gdouble     y,
                    GtkGesture *gesture)
{
  if (!self->symbols_visible)
    return;

  g_debug ("Long press on zero button");
  symbol_clicked (self, '+');
  gtk_gesture_set_state (GTK_GESTURE (gesture), GTK_EVENT_SEQUENCE_CLAIMED);
}


static void
cui_keypad_set_property (GObject      *object,
                         guint         property_id,
                         const GValue *value,
                         GParamSpec   *pspec)
{
  CuiKeypad *self = CUI_KEYPAD (object);

  switch (property_id) {
  case PROP_ROW_SPACING:
    cui_keypad_set_row_spacing (self, g_value_get_uint (value));
    break;
  case PROP_COLUMN_SPACING:
    cui_keypad_set_column_spacing (self, g_value_get_uint (value));
    break;
  case PROP_LETTERS_VISIBLE:
    cui_keypad_set_letters_visible (self, g_value_get_boolean (value));
    break;
  case PROP_SYMBOLS_VISIBLE:
    cui_keypad_set_symbols_visible (self, g_value_get_boolean (value));
    break;
  case PROP_ENTRY:
    cui_keypad_set_entry (self, g_value_get_object (value));
    break;
  case PROP_END_ACTION:
    cui_keypad_set_end_action (self, g_value_get_object (value));
    break;
  case PROP_START_ACTION:
    cui_keypad_set_start_action (self, g_value_get_object (value));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
cui_keypad_get_property (GObject    *object,
                         guint       property_id,
                         GValue     *value,
                         GParamSpec *pspec)
{
  CuiKeypad *self = CUI_KEYPAD (object);

  switch (property_id) {
  case PROP_ROW_SPACING:
    g_value_set_uint (value, cui_keypad_get_row_spacing (self));
    break;
  case PROP_COLUMN_SPACING:
    g_value_set_uint (value, cui_keypad_get_column_spacing (self));
    break;
  case PROP_LETTERS_VISIBLE:
    g_value_set_boolean (value, cui_keypad_get_letters_visible (self));
    break;
  case PROP_SYMBOLS_VISIBLE:
    g_value_set_boolean (value, cui_keypad_get_symbols_visible (self));
    break;
  case PROP_ENTRY:
    g_value_set_object (value, cui_keypad_get_entry (self));
    break;
  case PROP_START_ACTION:
    g_value_set_object (value, cui_keypad_get_start_action (self));
    break;
  case PROP_END_ACTION:
    g_value_set_object (value, cui_keypad_get_end_action (self));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
cui_keypad_finalize (GObject *object)
{
  CuiKeypad *self = CUI_KEYPAD (object);

  g_clear_object (&self->long_press_zero_gesture);
  g_clear_pointer (&self->re_separators, g_regex_unref);
  g_clear_pointer (&self->re_no_digits, g_regex_unref);
  g_clear_pointer (&self->re_no_digits_or_symbols, g_regex_unref);

  G_OBJECT_CLASS (cui_keypad_parent_class)->finalize (object);
}


static void
cui_keypad_class_init (CuiKeypadClass *klass)
{
  GObjectClass *object_class = G_OBJECT_CLASS (klass);
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->finalize = cui_keypad_finalize;

  object_class->set_property = cui_keypad_set_property;
  object_class->get_property = cui_keypad_get_property;

  /**
   * CuiKeypad:row-spacing:
   *
   * The amount of space between two consecutive rows.
   */
  props[PROP_ROW_SPACING] =
    g_param_spec_uint ("row-spacing",
                       "Row spacing",
                       "The amount of space between two consecutive rows",
                       0, G_MAXINT16, 6,
                       G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:column-spacing:
   *
   * The amount of space between two consecutive columns.
   */
  props[PROP_COLUMN_SPACING] =
    g_param_spec_uint ("column-spacing",
                       "Column spacing",
                       "The amount of space between two consecutive columns",
                       0, G_MAXINT16, 6,
                       G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:letters-visible:
   *
   * Whether the keypad should display the standard letters below the digits on
   * its buttons.
   */
  props[PROP_LETTERS_VISIBLE] =
    g_param_spec_boolean ("letters-visible",
                          "Letters visible",
                          "Whether the letters below the digits should be visible",
                          TRUE,
                          G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:symbols-visible:
   *
   * Whether the keypad should display the hash and asterisk buttons, and should
   * display the plus symbol at the bottom of its 0 button.
   */
  props[PROP_SYMBOLS_VISIBLE] =
    g_param_spec_boolean ("symbols-visible",
                          "Symbols visible",
                          "Whether the hash, plus, and asterisk symbols should be visible",
                          TRUE,
                          G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:entry:
   *
   * The entry widget connected to the keypad. See cui_keypad_set_entry() for details.
   */
  props[PROP_ENTRY] =
    g_param_spec_object ("entry",
                         "Entry",
                         "The entry widget connected to the keypad",
                         GTK_TYPE_ENTRY,
                         G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:end-action:
   *
   * The widget for the lower end corner of @self.
   */
  props[PROP_END_ACTION] =
    g_param_spec_object ("end-action",
                         "End action",
                         "The end action widget",
                         GTK_TYPE_WIDGET,
                         G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  /**
   * CuiKeypad:start-action:
   *
   * The widget for the lower start corner of @self.
   */
  props[PROP_START_ACTION] =
    g_param_spec_object ("start-action",
                         "Start action",
                         "The start action widget",
                         GTK_TYPE_WIDGET,
                         G_PARAM_READWRITE | G_PARAM_EXPLICIT_NOTIFY);

  g_object_class_install_properties (object_class, PROP_LAST_PROP, props);

  gtk_widget_class_set_template_from_resource (widget_class,
                                               "/org/gnome/CallUI/ui/cui-keypad.ui");
  gtk_widget_class_bind_template_child (widget_class, CuiKeypad, grid);
  gtk_widget_class_bind_template_child (widget_class, CuiKeypad, label_asterisk);
  gtk_widget_class_bind_template_child (widget_class, CuiKeypad, label_hash);
  gtk_widget_class_bind_template_child (widget_class, CuiKeypad, long_press_zero_gesture);

  gtk_widget_class_bind_template_callback (widget_class, button_clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, asterisk_button_clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, hash_button_clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, long_press_zero_cb);

  gtk_widget_class_set_accessible_role (widget_class, ATK_ROLE_DIAL);
  gtk_widget_class_set_css_name (widget_class, "cui-keypad");
}


static void
cui_keypad_init (CuiKeypad *self)
{
  g_autoptr (GError) error = NULL;

  self->row_spacing = 6;
  self->column_spacing = 6;
  self->letters_visible = TRUE;
  self->symbols_visible = TRUE;

  self->re_separators = g_regex_new ("(\\(0\\)|[-./[:blank:]()\\[\\]])", 0, 0, &error);
  if (!self->re_separators)
    g_warning ("Could not compile regex for visual separators: %s",
               error->message);

  g_clear_error (&error);
  self->re_no_digits = g_regex_new ("[^0-9]", 0, 0, &error);
  if (!self->re_no_digits)
    g_warning ("Could not compile regex for non-digits: %s",
               error->message);

  g_clear_error (&error);
  self->re_no_digits_or_symbols = g_regex_new ("[^0-9#*+]", 0, 0, &error);
  if (!self->re_no_digits_or_symbols)
    g_warning ("Could not compile regex for non-digits or non-symbols: %s",
               error->message);

  g_type_ensure (CUI_TYPE_KEYPAD_BUTTON);
  gtk_widget_init_template (GTK_WIDGET (self));

  gtk_widget_set_direction (GTK_WIDGET (self->grid), GTK_TEXT_DIR_LTR);
}


/**
 * cui_keypad_new:
 * @symbols_visible: whether the hash, plus, and asterisk symbols should be visible
 * @letters_visible: whether the letters below the digits should be visible
 *
 * Create a new #CuiKeypad widget.
 *
 * Returns: the newly created #CuiKeypad widget
 */
GtkWidget *
cui_keypad_new (gboolean symbols_visible,
                gboolean letters_visible)
{
  return g_object_new (CUI_TYPE_KEYPAD,
                       "symbols-visible", symbols_visible,
                       "letters-visible", letters_visible,
                       NULL);
}

/**
 * cui_keypad_set_row_spacing:
 * @self: a #CuiKeypad
 * @spacing: the amount of space to insert between rows
 *
 * Sets the amount of space between rows of @self.
 */
void
cui_keypad_set_row_spacing (CuiKeypad *self,
                            guint      spacing)
{
  g_return_if_fail (CUI_IS_KEYPAD (self));
  g_return_if_fail (spacing <= G_MAXINT16);

  if (self->row_spacing == spacing)
    return;

  self->row_spacing = spacing;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_ROW_SPACING]);
}


/**
 * cui_keypad_get_row_spacing:
 * @self: a #CuiKeypad
 *
 * Returns the amount of space between the rows of @self.
 *
 * Returns: the row spacing of @self
 */
guint
cui_keypad_get_row_spacing (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), 0);

  return self->row_spacing;
}


/**
 * cui_keypad_set_column_spacing:
 * @self: a #CuiKeypad
 * @spacing: the amount of space to insert between columns
 *
 * Sets the amount of space between columns of @self.
 */
void
cui_keypad_set_column_spacing (CuiKeypad *self,
                               guint      spacing)
{
  g_return_if_fail (CUI_IS_KEYPAD (self));
  g_return_if_fail (spacing <= G_MAXINT16);

  if (self->column_spacing == spacing)
    return;

  self->column_spacing = spacing;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_COLUMN_SPACING]);
}


/**
 * cui_keypad_get_column_spacing:
 * @self: a #CuiKeypad
 *
 * Returns the amount of space between the columns of @self.
 *
 * Returns: the column spacing of @self
 */
guint
cui_keypad_get_column_spacing (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), 0);

  return self->column_spacing;
}


/**
 * cui_keypad_set_letters_visible:
 * @self: a #CuiKeypad
 * @letters_visible: whether the letters below the digits should be visible
 *
 * Sets whether @self should display the standard letters below the digits on
 * its buttons.
 */
void
cui_keypad_set_letters_visible (CuiKeypad *self,
                                gboolean   letters_visible)
{
  g_return_if_fail (CUI_IS_KEYPAD (self));

  letters_visible = !!letters_visible;

  if (self->letters_visible == letters_visible)
    return;

  self->letters_visible = letters_visible;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_LETTERS_VISIBLE]);
}


/**
 * cui_keypad_get_letters_visible:
 * @self: a #CuiKeypad
 *
 * Returns whether @self should display the standard letters below the digits on
 * its buttons.
 *
 * Returns: whether the letters below the digits should be visible
 */
gboolean
cui_keypad_get_letters_visible (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), FALSE);

  return self->letters_visible;
}


/**
 * cui_keypad_set_symbols_visible:
 * @self: a #CuiKeypad
 * @symbols_visible: whether the hash, plus, and asterisk symbols should be visible
 *
 * Sets whether @self should display the hash and asterisk buttons, and should
 * display the plus symbol at the bottom of its 0 button.
 */
void
cui_keypad_set_symbols_visible (CuiKeypad *self,
                                gboolean   symbols_visible)
{
  g_return_if_fail (CUI_IS_KEYPAD (self));

  symbols_visible = !!symbols_visible;

  if (self->symbols_visible == symbols_visible)
    return;

  self->symbols_visible = symbols_visible;

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_SYMBOLS_VISIBLE]);
}


/**
 * cui_keypad_get_symbols_visible:
 * @self: a #CuiKeypad
 *
 * Returns whether @self should display the standard letters below the digits on
 * its buttons.
 *
 * Returns Whether @self should display the hash and asterisk buttons, and
 * should display the plus symbol at the bottom of its 0 button.
 *
 * Returns: whether the hash, plus, and asterisk symbols should be visible
 */
gboolean
cui_keypad_get_symbols_visible (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), FALSE);

  return self->symbols_visible;
}


/**
 * cui_keypad_set_entry:
 * @self: a #CuiKeypad
 * @entry: (nullable): a #GtkEntry
 *
 * Binds @entry to @self and blocks any input which wouldn't be possible to type
 * with with the keypad.
 */
void
cui_keypad_set_entry (CuiKeypad *self,
                      GtkEntry  *entry)
{
  g_return_if_fail (CUI_IS_KEYPAD (self));
  g_return_if_fail (entry == NULL || GTK_IS_ENTRY (entry));

  if (entry == self->entry)
    return;

  g_clear_object (&self->entry);

  if (entry) {
    self->entry = g_object_ref (entry);

    gtk_widget_show (GTK_WIDGET (self->entry));
    /* Workaround: To keep the osk closed
     * https://gitlab.gnome.org/GNOME/gtk/merge_requests/978#note_546576 */
    g_object_set (self->entry, "im-module", "gtk-im-context-none", NULL);

    g_signal_connect_swapped (G_OBJECT (self->entry),
                              "insert-text",
                              G_CALLBACK (insert_text_cb),
                              self);
  }

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_ENTRY]);
}


/**
 * cui_keypad_get_entry:
 * @self: a #CuiKeypad
 *
 * Get the connected entry. See cui_keypad_set_entry() for details.
 *
 * Returns: (transfer none): the set #GtkEntry or %NULL if no widget was set
 */
GtkEntry *
cui_keypad_get_entry (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), NULL);

  return self->entry;
}


/**
 * cui_keypad_set_start_action:
 * @self: a #CuiKeypad
 * @start_action: (nullable): the start action widget
 *
 * Sets the widget for the lower left corner of @self.
 */
void
cui_keypad_set_start_action (CuiKeypad *self,
                             GtkWidget *start_action)
{
  GtkWidget *old_widget;

  g_return_if_fail (CUI_IS_KEYPAD (self));
  g_return_if_fail (start_action == NULL || GTK_IS_WIDGET (start_action));

  old_widget = gtk_grid_get_child_at (GTK_GRID (self->grid), 0, 3);

  if (old_widget == start_action)
    return;

  if (old_widget != NULL)
    gtk_container_remove (GTK_CONTAINER (self->grid), old_widget);

  if (start_action != NULL)
    gtk_grid_attach (GTK_GRID (self->grid), start_action, 0, 3, 1, 1);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_START_ACTION]);
}


/**
 * cui_keypad_get_start_action:
 * @self: a #CuiKeypad
 *
 * Returns the widget for the lower left corner @self.
 *
 * Returns: (transfer none) (nullable): the start action widget
 */
GtkWidget *
cui_keypad_get_start_action (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), NULL);

  return gtk_grid_get_child_at (GTK_GRID (self->grid), 0, 3);
}


/**
 * cui_keypad_set_end_action:
 * @self: a #CuiKeypad
 * @end_action: (nullable): the end action widget
 *
 * Sets the widget for the lower right corner of @self.
 */
void
cui_keypad_set_end_action (CuiKeypad *self,
                           GtkWidget *end_action)
{
  GtkWidget *old_widget;

  g_return_if_fail (CUI_IS_KEYPAD (self));
  g_return_if_fail (end_action == NULL || GTK_IS_WIDGET (end_action));

  old_widget = gtk_grid_get_child_at (GTK_GRID (self->grid), 2, 3);

  if (old_widget == end_action)
    return;

  if (old_widget != NULL)
    gtk_container_remove (GTK_CONTAINER (self->grid), old_widget);

  if (end_action != NULL)
    gtk_grid_attach (GTK_GRID (self->grid), end_action, 2, 3, 1, 1);

  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_END_ACTION]);
}


/**
 * cui_keypad_get_end_action:
 * @self: a #CuiKeypad
 *
 * Returns the widget for the lower right corner of @self.
 *
 * Returns: (transfer none) (nullable): the end action widget
 */
GtkWidget *
cui_keypad_get_end_action (CuiKeypad *self)
{
  g_return_val_if_fail (CUI_IS_KEYPAD (self), NULL);

  return gtk_grid_get_child_at (GTK_GRID (self->grid), 2, 3);
}
