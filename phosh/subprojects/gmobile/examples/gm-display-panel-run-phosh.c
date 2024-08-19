/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: GPL-3-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

/* This examples launches phosh and phoc emulationg the display
   of the given device tree compatible */

#define GMOBILE_USE_UNSTABLE_API
#include "gmobile.h"

#include <glib-unix.h>
#include <glib/gstdio.h>

#include <gio/gio.h>
#include <glib/gprintf.h>

#define PHOSH_BIN "/usr/libexec/phosh"

GMainLoop   *loop;
GSubprocess *phoc;

G_NORETURN static void
print_version (void)
{
  g_printf ("gm-emu-device--panel %s\n", GM_VERSION);
  exit (0);
}


static gboolean
on_shutdown_signal (gpointer unused)
{
  g_autoptr (GError) err = NULL;
  gboolean success;

  success = g_subprocess_wait (phoc, NULL, &err);
  if (!success)
    g_warning ("Failed to terminate phoc: %s", err->message);

  g_main_loop_quit (loop);

  return G_SOURCE_REMOVE;
}


static char *
write_phoc_ini (GmDisplayPanel *panel, gdouble scale)
{
  g_autoptr (GError) err = NULL;
  g_autoptr (GString) content = g_string_new ("[output:WL-1]\n");
  g_autofree char *phoc_ini = NULL;
  int xres = gm_display_panel_get_x_res (panel);
  int yres = gm_display_panel_get_y_res (panel);
  int fd;

  g_string_append_printf (content, "mode = %dx%d\n", xres, yres);
  g_string_append_printf (content, "scale = %.2f\n", scale);
  fd = g_file_open_tmp ("phoc_XXXXXX.ini", &phoc_ini, &err);
  if (fd < 0) {
    g_critical ("Failed to open %s: %s", phoc_ini, err->message);
    return NULL;
  }

  if (write (fd, content->str, strlen (content->str)) < 0) {
    g_critical ("Failed to write %s", strerror (errno));
    return NULL;
  }

  return g_steal_pointer (&phoc_ini);
}


int main (int argc, char **argv)
{
  g_autoptr (GOptionContext) opt_context = NULL;
  gboolean version = FALSE;
  g_autoptr (GError) err = NULL;
  g_autoptr (GmDeviceInfo) info = NULL;
  g_auto (GStrv) compatibles = NULL;
  GmDisplayPanel *panel = NULL;
  GStrv compatibles_opt = NULL;
  g_autofree char *phoc_ini = NULL;
  g_autoptr (GSubprocessLauncher) phoc_launcher = NULL;
  double scale_opt = 1.0;
  const char *phosh_bin;

  const GOptionEntry options [] = {
    {"compatible", 'c', 0, G_OPTION_ARG_STRING_ARRAY, &compatibles_opt,
     "Device tree compatibles to use for panel lookup ", NULL},
    {"scale", 's', 0, G_OPTION_ARG_DOUBLE, &scale_opt,
     "The display scale", NULL },
    {"version", 0, 0, G_OPTION_ARG_NONE, &version,
     "Show version information", NULL},
    { NULL, 0, 0, G_OPTION_ARG_NONE, NULL, NULL, NULL }
  };

  opt_context = g_option_context_new ("- emulate display panel");
  g_option_context_add_main_entries (opt_context, options, NULL);
  if (!g_option_context_parse (opt_context, &argc, &argv, &err)) {
    g_warning ("%s", err->message);
    g_clear_error (&err);
    return EXIT_FAILURE;
  }

  if (version)
    print_version ();

  if (compatibles_opt && compatibles_opt[0]) {
    compatibles = g_strdupv (compatibles_opt);
  } else {
    compatibles = gm_device_tree_get_compatibles (NULL, &err);
    if (compatibles == NULL) {
      g_critical ("Failed to get compatibles: %s", err->message);
      return EXIT_FAILURE;
    }
  }

  info = gm_device_info_new ((const char *const *)compatibles);
  panel = gm_device_info_get_display_panel (info);
  if (panel == NULL) {
    g_critical ("Failed to find any panel");
    return EXIT_FAILURE;
  }

  phoc_ini = write_phoc_ini (panel, scale_opt);
  if (!phoc_ini)
    return EXIT_FAILURE;

  g_message ("Using %s as phoc config", phoc_ini);

  phosh_bin = g_getenv ("PHOSH_BIN") ?: PHOSH_BIN;
  phoc_launcher = g_subprocess_launcher_new (G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP);
  g_subprocess_launcher_set_environ (phoc_launcher, NULL);
  g_subprocess_launcher_setenv (phoc_launcher, "WLR_BACKENDS", "wayland", TRUE);
  g_subprocess_launcher_setenv (phoc_launcher, "GSETTINGS_BACKEND", "memory", TRUE);
  g_subprocess_launcher_setenv (phoc_launcher, "PHOC_DEBUG", "cutouts", TRUE);
  g_subprocess_launcher_setenv (phoc_launcher, "PHOSH_DEBUG", "fake-builtin", TRUE);
  g_subprocess_launcher_setenv (phoc_launcher, "G_MESSAGES_DEBUG", "phosh-layout-manager", TRUE);
  if (compatibles_opt && compatibles_opt[0]) {
    g_autofree char *opt = g_strjoinv (",", compatibles_opt);
    g_subprocess_launcher_setenv (phoc_launcher, "GMOBILE_DT_COMPATIBLES", opt, TRUE);
  }
  g_subprocess_launcher_setenv (phoc_launcher, "WLR_BACKENDS", "wayland", TRUE);
  phoc = g_subprocess_launcher_spawnv (phoc_launcher,
                                       (const char * const [])
                                       { "phoc", "-C", phoc_ini,
                                         "-E", phosh_bin, NULL },
                                       &err);
  g_unix_signal_add (SIGTERM, on_shutdown_signal, NULL);
  g_unix_signal_add (SIGINT, on_shutdown_signal, NULL);

  loop = g_main_loop_new (NULL, FALSE);

  g_message  ("Launching phosh and phoc, hit CTRL-C to quit");
  g_main_loop_run (loop);

  g_unlink (phoc_ini);

  g_clear_object (&phoc);
  g_clear_object (&loop);

  return EXIT_SUCCESS;
}
