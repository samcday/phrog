/*
 * Copyright (C) 2018 Purism SPC
 *               2024-2025 Phosh.mobi e.V.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#define G_LOG_DOMAIN "phosh-activity"

#include "phosh-config.h"
#include "activity.h"
#include "shell-priv.h"
#include "swipe-away-bin.h"
#include "thumbnail.h"
#include "util.h"
#include "app-grid-button.h"

#include <gio/gdesktopappinfo.h>

/**
 * PhoshActivity:
 *
 * An app in the favorites overview
 *
 * The #PhoshActivity is used to select a running application in the overview.
 */

/* Icons actually sized according to the pixel-size set in the template */
#define ACTIVITY_ICON_SIZE -1

enum {
  CLICKED,
  CLOSED,
  FULLSCREENED,
  RESIZED,
  N_SIGNALS
};
static guint signals[N_SIGNALS] = { 0 };

enum {
  PROP_0,
  PROP_APP_ID,
  PROP_PARENT_APP_ID,
  PROP_MAXIMIZED,
  PROP_FULLSCREEN,
  PROP_WIN_WIDTH,
  PROP_WIN_HEIGHT,
  LAST_PROP,
};
static GParamSpec *props[LAST_PROP];

typedef struct {
  GtkWidget       *swipe_bin;
  GtkWidget       *icon;
  GtkWidget       *box;
  GtkWidget       *revealer_close;
  GtkWidget       *revealer_unfullscreen;
  GtkWidget       *btn_close;
  GtkWidget       *btn_unfullscreen;
  GtkPicture      *preview;
  GtkWidget       *button;

  gboolean         maximized;
  gboolean         fullscreen;
  int              win_width;
  int              win_height;

  char            *app_id;
  char            *parent_app_id;

  GdkTexture      *texture;

  gboolean         hovering;
  guint            remove_timeout_id;
  GtkAllocation    allocation;
} PhoshActivityPrivate;


struct _PhoshActivity {
  GtkWidget parent;
};

G_DEFINE_TYPE_WITH_PRIVATE(PhoshActivity, phosh_activity, GTK_TYPE_WIDGET)


static void
set_fullscreen (PhoshActivity *self, gboolean fullscreen)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  priv->fullscreen = fullscreen;
  phosh_util_toggle_style_class (GTK_WIDGET (self), "phosh-fullscreen", priv->fullscreen);

  gtk_revealer_set_reveal_child (GTK_REVEALER (priv->revealer_unfullscreen), priv->fullscreen);
}


static void
set_win_width (PhoshActivity *self, int width)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  if (width == priv->win_width)
    return;

  priv->win_width = width;
  gtk_widget_queue_resize (GTK_WIDGET (self));
  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_WIN_WIDTH]);
}


static void
set_win_height (PhoshActivity *self, int height)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  if (height == priv->win_height)
    return;

  priv->win_height = height;
  gtk_widget_queue_resize (GTK_WIDGET (self));
  g_object_notify_by_pspec (G_OBJECT (self), props[PROP_WIN_HEIGHT]);
}


static void
phosh_activity_set_property (GObject      *object,
                             guint         property_id,
                             const GValue *value,
                             GParamSpec   *pspec)
{
  PhoshActivity *self = PHOSH_ACTIVITY (object);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  switch (property_id) {
  case PROP_APP_ID:
    g_free (priv->app_id);
    priv->app_id = g_value_dup_string (value);
    break;
  case PROP_PARENT_APP_ID:
    g_free (priv->parent_app_id);
    priv->parent_app_id = g_value_dup_string (value);
    break;
  case PROP_MAXIMIZED:
    priv->maximized = g_value_get_boolean (value);
    phosh_util_toggle_style_class (GTK_WIDGET (self), "phosh-maximized", priv->maximized);
    break;
  case PROP_FULLSCREEN:
    set_fullscreen (self, g_value_get_boolean (value));
    break;
  case PROP_WIN_WIDTH:
    set_win_width (self, g_value_get_int (value));
    break;
  case PROP_WIN_HEIGHT:
    set_win_height (self, g_value_get_int (value));
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
phosh_activity_get_property (GObject    *object,
                             guint       property_id,
                             GValue     *value,
                             GParamSpec *pspec)
{
  PhoshActivity *self = PHOSH_ACTIVITY (object);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  switch (property_id) {
  case PROP_APP_ID:
    g_value_set_string (value, priv->app_id);
    break;
  case PROP_PARENT_APP_ID:
    g_value_set_string (value, priv->parent_app_id);
    break;
  case PROP_MAXIMIZED:
    g_value_set_boolean (value, priv->maximized);
    break;
  case PROP_FULLSCREEN:
    g_value_set_boolean (value, priv->fullscreen);
    break;
  case PROP_WIN_WIDTH:
    g_value_set_int (value, priv->win_width);
    break;
  case PROP_WIN_HEIGHT:
    g_value_set_int (value, priv->win_height);
    break;
  default:
    G_OBJECT_WARN_INVALID_PROPERTY_ID (object, property_id, pspec);
    break;
  }
}


static void
clicked_cb (PhoshActivity *self)
{
  g_signal_emit (self, signals[CLICKED], 0);
}


static void
closed_cb (PhoshActivity *self)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  phosh_swipe_away_bin_remove (PHOSH_SWIPE_AWAY_BIN (priv->swipe_bin));
}


static void
on_unfullscreen_clicked (PhoshActivity *self)
{
  g_signal_emit (self, signals[FULLSCREENED], 0, FALSE);
  g_signal_emit (self, signals[CLICKED], 0);
}


static void
on_remove_timeout (gpointer data)
{
  PhoshActivity *self = PHOSH_ACTIVITY (data);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  phosh_swipe_away_bin_undo (PHOSH_SWIPE_AWAY_BIN (priv->swipe_bin));

  priv->remove_timeout_id = 0;
}


static void
removed_cb (PhoshActivity *self)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  if (priv->remove_timeout_id)
    g_source_remove (priv->remove_timeout_id);

  priv->remove_timeout_id = g_timeout_add_seconds_once (1, on_remove_timeout, self);
  g_source_set_name_by_id (priv->remove_timeout_id, "[phosh] remove_timeout_id");

  g_signal_emit (self, signals[CLOSED], 0);
}


static float
get_scale (PhoshActivity *self)
{
  float scale;
  int width, height, image_width, image_height;
  PhoshActivityPrivate *priv;

  priv = phosh_activity_get_instance_private (self);
  width = gtk_widget_get_width (GTK_WIDGET (priv->preview));
  height = gtk_widget_get_height (GTK_WIDGET (priv->preview));

  if (!priv->texture)
    return 1.0;

  image_width = gdk_texture_get_width (priv->texture);
  image_height = gdk_texture_get_height (priv->texture);

  scale = width / (float)image_width;

  if (height / (float)image_height < scale)
    scale = height / (float)image_height;

  return scale;
}


static void
phosh_activity_constructed (GObject *object)
{
  PhoshActivity *self = PHOSH_ACTIVITY (object);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);
  g_autoptr (GDesktopAppInfo) app_info = NULL;
  GIcon *icon = NULL;

  app_info = phosh_get_desktop_app_info_for_app_id (priv->app_id);
  if (app_info)
    icon = g_app_info_get_icon (G_APP_INFO (app_info));

  if (!icon && priv->parent_app_id) {
    app_info = phosh_get_desktop_app_info_for_app_id (priv->parent_app_id);
    if (app_info)
      icon = g_app_info_get_icon (G_APP_INFO (app_info));
  }

  if (icon) {
    gtk_image_set_from_gicon (GTK_IMAGE (priv->icon), icon);
  } else {
    gtk_image_set_from_icon_name (GTK_IMAGE (priv->icon), PHOSH_APP_UNKNOWN_ICON);
    gtk_widget_add_css_class (GTK_WIDGET (self), "phosh-empty");
  }

  G_OBJECT_CLASS (phosh_activity_parent_class)->constructed (object);
}


static void
phosh_activity_dispose (GObject *object)
{
  PhoshActivity *self = PHOSH_ACTIVITY (object);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  g_clear_object (&priv->texture);
  g_clear_handle_id (&priv->remove_timeout_id, g_source_remove);

  gtk_widget_dispose_template (GTK_WIDGET (object), PHOSH_TYPE_ACTIVITY);

  G_OBJECT_CLASS (phosh_activity_parent_class)->dispose (object);
}


static void
phosh_activity_finalize (GObject *object)
{
  PhoshActivity *self = PHOSH_ACTIVITY (object);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  g_free (priv->parent_app_id);
  g_free (priv->app_id);

  G_OBJECT_CLASS (phosh_activity_parent_class)->finalize (object);
}


static GtkSizeRequestMode
phosh_activity_get_request_mode (GtkWidget *widget)
{
  return GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT;
}


static void
phosh_activity_get_preferred_height (GtkWidget *widget,
                                     int       *min,
                                     int       *nat)
{
  PhoshActivityPrivate *priv;
  GtkWidget *child;
  int smallest = 0;
  int box_smallest = 0;
  int parent_nat;

  g_return_if_fail (PHOSH_IS_ACTIVITY (widget));
  priv = phosh_activity_get_instance_private (PHOSH_ACTIVITY (widget));

  child = gtk_widget_get_first_child (widget);
  if (child)
    gtk_widget_measure (child, GTK_ORIENTATION_VERTICAL, -1, &smallest, &parent_nat, NULL, NULL);
  gtk_widget_measure (priv->box, GTK_ORIENTATION_HORIZONTAL, -1, &box_smallest, NULL, NULL, NULL);

  smallest = MAX (smallest, box_smallest);

  if (min)
    *min = smallest;

  if (nat)
    *nat = smallest;
}


static void
phosh_activity_get_preferred_width (GtkWidget *widget,
                                    int       *min,
                                    int       *nat)
{
  PhoshActivityPrivate *priv;
  GtkWidget *child;
  int smallest = 0;
  int box_smallest = 0;
  int size;
  int parent_nat;
  int margin_start, margin_end;

  g_return_if_fail (PHOSH_IS_ACTIVITY (widget));
  priv = phosh_activity_get_instance_private (PHOSH_ACTIVITY (widget));

  child = gtk_widget_get_first_child (widget);
  if (child)
    gtk_widget_measure (child, GTK_ORIENTATION_HORIZONTAL, -1, &smallest, &parent_nat, NULL, NULL);

  gtk_widget_measure (priv->box, GTK_ORIENTATION_HORIZONTAL, -1, &box_smallest, NULL, NULL, NULL);

  smallest = MAX (smallest, box_smallest);

  margin_start = gtk_widget_get_margin_start (GTK_WIDGET (priv->preview));
  margin_end = gtk_widget_get_margin_end (GTK_WIDGET (priv->preview));

  size = smallest + margin_start + margin_end;

  if (min)
    *min = size;

  if (nat)
    *nat = size;
}


static void
phosh_activity_get_preferred_width_for_height (GtkWidget *widget,
                                               int        height,
                                               int       *min,
                                               int       *nat)
{
  PhoshActivityPrivate *priv;
  GtkWidget *child;
  int smallest = 0;
  int box_smallest = 0;
  int size;
  int parent_nat;
  int margin_start, margin_end, margin_top, margin_bottom;
  double aspect_ratio;

  g_return_if_fail (PHOSH_IS_ACTIVITY (widget));
  priv = phosh_activity_get_instance_private (PHOSH_ACTIVITY (widget));

  child = gtk_widget_get_first_child (widget);
  if (child)
    gtk_widget_measure (child, GTK_ORIENTATION_HORIZONTAL, height, &smallest, &parent_nat, NULL, NULL);

  gtk_widget_measure (priv->box, GTK_ORIENTATION_HORIZONTAL, height, &box_smallest, NULL, NULL, NULL);

  smallest = MAX (smallest, box_smallest);

  margin_start = gtk_widget_get_margin_start (GTK_WIDGET (priv->preview));
  margin_end = gtk_widget_get_margin_end (GTK_WIDGET (priv->preview));
  margin_top = gtk_widget_get_margin_top (GTK_WIDGET (priv->preview));
  margin_bottom = gtk_widget_get_margin_bottom (GTK_WIDGET (priv->preview));

  aspect_ratio = (double) priv->win_width / priv->win_height;
  size = MAX (smallest,
              (height - margin_top - margin_bottom) * aspect_ratio) + margin_start + margin_end;

  if (min)
    *min = size;

  if (nat)
    *nat = size;
}


static void
phosh_activity_get_preferred_height_for_width (GtkWidget *widget,
                                               int        width,
                                               int       *min,
                                               int       *nat)
{
  phosh_activity_get_preferred_height (widget, min, nat);
}


static void
phosh_activity_measure (GtkWidget      *widget,
                        GtkOrientation  orientation,
                        int             for_size,
                        int            *minimum,
                        int            *natural,
                        int            *minimum_baseline,
                        int            *natural_baseline)
{
  if (orientation == GTK_ORIENTATION_HORIZONTAL && for_size != -1)
    phosh_activity_get_preferred_width_for_height (widget, for_size, minimum, natural);
  else if (orientation == GTK_ORIENTATION_HORIZONTAL && for_size == -1)
    phosh_activity_get_preferred_width (widget, minimum, natural);
  else if (orientation == GTK_ORIENTATION_VERTICAL && for_size != -1)
    phosh_activity_get_preferred_height_for_width (widget, for_size, minimum, natural);
  else if (orientation == GTK_ORIENTATION_VERTICAL && for_size == -1)
    phosh_activity_get_preferred_height (widget, minimum, natural);
  else
    g_assert_not_reached ();
}


static void
phosh_activity_size_allocate (GtkWidget *widget, int width, int height, int baseline)
{
  GtkWidget *child = gtk_widget_get_first_child (widget);
  GtkAllocation allocation = {0, 0, width, height};
  PhoshActivity *self = PHOSH_ACTIVITY (widget);
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);
  gboolean changed = width != priv->allocation.width || height != priv->allocation.height;

  priv->allocation.width = width;
  priv->allocation.height = height;

  if (changed)
    g_signal_emit (self, signals[RESIZED], 0, &priv->allocation);

  if (child)
    gtk_widget_size_allocate (child, &allocation, baseline);
}


static void
set_hovering (PhoshActivity *self,
              gboolean       hovering)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  if (hovering == priv->hovering)
    return;

  priv->hovering = hovering;

  /* Revealer won't animate if not mapped, show it preemptively */
  if (hovering)
    gtk_widget_set_visible (priv->revealer_close, TRUE);

  gtk_revealer_set_reveal_child (GTK_REVEALER (priv->revealer_close), hovering);
}


static void
on_motion_enter (PhoshActivity      *self,
                 double              x,
                 double              y,
                 GtkEventController *controller)
{
  /* enter-notify never happens on touch, so we don't need to check it */
  set_hovering (self, TRUE);
}


static void
on_motion_leave (PhoshActivity      *self,
                 double              x,
                 double              y,
                 GtkEventController *controller)
{
  set_hovering (self, FALSE);
}


static void
on_motion (PhoshActivity      *self,
           double              x,
           double              y,
           GtkEventController *controller)
{
  GdkDevice *device = gtk_event_controller_get_current_event_device (controller);
  GdkInputSource source = gdk_device_get_source (device);

  if (source == GDK_SOURCE_TOUCHSCREEN)
    return;

  set_hovering (self, TRUE);
}


static gboolean
on_key_pressed (GtkWidget          *self,
                guint               keyval,
                guint               keycode,
                GdkModifierType    *state,
                GtkEventController *controller)
{
  gboolean handled = FALSE;
  PhoshActivityPrivate *priv;

  g_return_val_if_fail (PHOSH_IS_ACTIVITY (self), FALSE);
  priv = phosh_activity_get_instance_private (PHOSH_ACTIVITY (self));

  switch (keyval) {
  case GDK_KEY_Return:
    g_signal_emit_by_name (priv->button, "clicked");
    handled = TRUE;
    break;
  default:
    /* nothing to do */
    break;
  }

  return handled;
}


static void
phosh_activity_unmap (GtkWidget *widget)
{
  set_hovering (PHOSH_ACTIVITY (widget), FALSE);

  GTK_WIDGET_CLASS (phosh_activity_parent_class)->unmap (widget);
}


static void
phosh_activity_class_init (PhoshActivityClass *klass)
{
  GObjectClass *object_class = (GObjectClass *)klass;
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  object_class->constructed = phosh_activity_constructed;
  object_class->dispose = phosh_activity_dispose;
  object_class->finalize = phosh_activity_finalize;

  object_class->set_property = phosh_activity_set_property;
  object_class->get_property = phosh_activity_get_property;

  widget_class->get_request_mode = phosh_activity_get_request_mode;
  widget_class->measure = phosh_activity_measure;
  widget_class->size_allocate = phosh_activity_size_allocate;
  widget_class->unmap = phosh_activity_unmap;

  /**
   * PhoshActivity:app-id:
   *
   * The app-id of the activity
   */
  props[PROP_APP_ID] =
    g_param_spec_string ("app-id", "", "",
                         "",
                         G_PARAM_CONSTRUCT_ONLY | G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * PhoshActivity:parent-app-id:
   *
   * The app-id of the parent activity (if any)
   */
  props[PROP_PARENT_APP_ID] =
    g_param_spec_string ("parent-app-id", "", "",
                         NULL,
                         G_PARAM_CONSTRUCT_ONLY | G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * PhoshActivity:maximized:
   *
   * Whether the window is maximized
   */
  props[PROP_MAXIMIZED] =
    g_param_spec_boolean ("maximized", "", "",
                          FALSE,
                          G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * PhoshActivity:fullscreen:
   *
   * Whether the window is presented fullscreen
   */
  props[PROP_FULLSCREEN] =
    g_param_spec_boolean ("fullscreen", "", "",
                          FALSE,
                          G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS);
  /**
   * PhoshActivity:win-width:
   *
   * The window's width
   */
  props[PROP_WIN_WIDTH] =
    g_param_spec_int ("win-width", "", "",
                      0, G_MAXINT, 300,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS | G_PARAM_EXPLICIT_NOTIFY);
  /**
   * PhoshActivity:win-height:
   *
   * The window's height
   */
  props[PROP_WIN_HEIGHT] =
    g_param_spec_int ("win-height", "", "",
                      0, G_MAXINT, 300,
                      G_PARAM_READWRITE | G_PARAM_STATIC_STRINGS | G_PARAM_EXPLICIT_NOTIFY);

  g_object_class_install_properties (object_class, LAST_PROP, props);

  signals[CLICKED] = g_signal_new ("clicked",
                                   G_TYPE_FROM_CLASS (klass),
                                   G_SIGNAL_RUN_LAST, 0, NULL, NULL,
                                   NULL,
                                   G_TYPE_NONE,
                                   0);

  signals[CLOSED] = g_signal_new ("closed",
                                  G_TYPE_FROM_CLASS (klass),
                                  G_SIGNAL_RUN_LAST, 0, NULL, NULL,
                                  NULL,
                                  G_TYPE_NONE,
                                  0);
  /**
   * PhoshActivity::fullscreened
   * @self: The activity
   * @fullscreen: Whether the activity should be fullscreened or
   *   unfullscreened
   *
   * The fullscreen state of the activity should be changed.
   */
  signals[FULLSCREENED] = g_signal_new ("fullscreened",
                                        G_TYPE_FROM_CLASS (klass),
                                        G_SIGNAL_RUN_LAST, 0, NULL, NULL,
                                        NULL,
                                        G_TYPE_NONE, 1,
                                        G_TYPE_BOOLEAN);

  signals[RESIZED] = g_signal_new ("resized",
                                   G_TYPE_FROM_CLASS (klass),
                                   G_SIGNAL_RUN_LAST, 0, NULL, NULL,
                                   NULL,
                                   G_TYPE_NONE,
                                   1,
                                   GDK_TYPE_RECTANGLE | G_SIGNAL_TYPE_STATIC_SCOPE);

  g_type_ensure (PHOSH_TYPE_SWIPE_AWAY_BIN);

  gtk_widget_class_set_template_from_resource (widget_class, "/mobi/phosh/ui/activity.ui");

  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, btn_close);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, btn_unfullscreen);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, button);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, preview);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, swipe_bin);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, icon);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, box);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, revealer_close);
  gtk_widget_class_bind_template_child_private (widget_class, PhoshActivity, revealer_unfullscreen);
  gtk_widget_class_bind_template_callback (widget_class, clicked_cb);
  gtk_widget_class_bind_template_callback (widget_class, closed_cb);
  gtk_widget_class_bind_template_callback (widget_class, on_unfullscreen_clicked);
  gtk_widget_class_bind_template_callback (widget_class, removed_cb);
  gtk_widget_class_bind_template_callback (widget_class, on_key_pressed);
  gtk_widget_class_bind_template_callback (widget_class, on_motion_enter);
  gtk_widget_class_bind_template_callback (widget_class, on_motion_leave);
  gtk_widget_class_bind_template_callback (widget_class, on_motion);

  gtk_widget_class_set_css_name (widget_class, "phosh-activity");
}


static void
phosh_activity_init (PhoshActivity *self)
{
  PhoshActivityPrivate *priv = phosh_activity_get_instance_private (self);

  gtk_widget_init_template (GTK_WIDGET (self));

  priv->win_height = 300;
  priv->win_width = 300;
}


GtkWidget *
phosh_activity_new (const char *app_id)
{
  return g_object_new (PHOSH_TYPE_ACTIVITY,
                       "app-id", app_id,
                       NULL);
}


const char *
phosh_activity_get_app_id (PhoshActivity *self)
{
  PhoshActivityPrivate *priv;

  g_return_val_if_fail (PHOSH_IS_ACTIVITY (self), NULL);
  priv = phosh_activity_get_instance_private (self);

  return priv->app_id;
}

/**
 * phosh_activity_set_thumbnail:
 * @self: the activity
 * @thumbnail:(transfer none): the thumbnail
 *
 * Sets the given thumbnail
 */
void
phosh_activity_set_thumbnail (PhoshActivity *self, PhoshThumbnail *thumbnail)
{
  PhoshActivityPrivate *priv;
  gpointer data;
  guint w, width, height, stride, margin;
  gsize size;
  g_autoptr (GBytes) bytes = NULL;
  float scale;

  g_return_if_fail (PHOSH_IS_ACTIVITY (self));
  priv = phosh_activity_get_instance_private (self);

  data = phosh_thumbnail_get_image (thumbnail);
  phosh_thumbnail_get_size (thumbnail, &width, &height, &stride);

  size = stride * height;
  /* TODO: pass shm directly */
  bytes = g_bytes_new_take (g_memdup2 (data, size), size);
  g_clear_object (&priv->texture);
  priv->texture = GDK_TEXTURE (gdk_memory_texture_new (width, height, GDK_MEMORY_A8R8G8B8, bytes, stride));
  gtk_picture_set_paintable (priv->preview, GDK_PAINTABLE (priv->texture));

  phosh_util_toggle_style_class (GTK_WIDGET (self), "phosh-empty", FALSE);

  /* Make sure buttons are over the thumbnail */
  w = gtk_widget_get_width (GTK_WIDGET (self));
  scale = get_scale (self);
  margin = w ? (w - (width * scale)) / 2 : 0;
  gtk_widget_set_margin_start (priv->btn_unfullscreen, margin);
  gtk_widget_set_margin_end (priv->btn_close, margin);
}

void
phosh_activity_get_thumbnail_allocation (PhoshActivity *self, GtkAllocation *allocation)
{
  PhoshActivityPrivate *priv;

  g_return_if_fail (allocation);
  g_return_if_fail (PHOSH_IS_ACTIVITY (self));
  priv = phosh_activity_get_instance_private (self);

  *allocation = priv->allocation;
}
