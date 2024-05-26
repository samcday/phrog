/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-device-tree.h"

#include <gio/gio.h>
#include <glib.h>

#define DT_COMPATIBLE_PATH "firmware/devicetree/base/compatible"

static GStrv
get_env_compatibles (void)
{
  const char *env = g_getenv ("GMOBILE_DT_COMPATIBLES");

  if (env)
    return g_strsplit(env, ":", -1);

  return NULL;
}

/**
 * gm_device_tree_get_compatibles:
 * @sysfs_root: Path where /sys is mounted. Defaults to `/sys` if %NULL is passed.
 * @err: return location for error or %NULL
 *
 * Read compatible machine types from
 * `sysfs_root/firmware/devicetree/base/compatible` on Linux.
 * If the path doesn't exist or host is not Linux return %NULL.
 *
 * For debugging purposes `GMOBILE_DT_COMPATIBLES` can be set to a `:`
 * separated list of compatibles which will be returned instead.
 *
 * Returns:(transfer full): compatible machine types or %NULL
 *
 * Since: 0.0.1
 */
GStrv
gm_device_tree_get_compatibles (const char *sysfs_root, GError **err)
{
  GStrv env_compatibles;
#ifdef __linux__
  gsize len;
  g_autoptr (GPtrArray) parts = NULL;
  g_autofree char *compatible_path = NULL;
  g_autofree gchar *compatibles = NULL;
  const char *comp;

  g_return_val_if_fail (err == NULL || *err == NULL, NULL);

  env_compatibles = get_env_compatibles();
  if (env_compatibles)
    return env_compatibles;

  compatible_path = g_build_filename (sysfs_root ?: "/sys",
                                      DT_COMPATIBLE_PATH, NULL);
  if (g_file_test (compatible_path, (G_FILE_TEST_EXISTS)) == FALSE) {
    g_set_error (err, G_IO_ERROR, G_IO_ERROR_NOT_FOUND,
                 "%s not found", compatible_path);
    return NULL;
  }

  if (!g_file_get_contents (compatible_path, &compatibles, &len, err)) {
    return NULL;
  }

  parts = g_ptr_array_new ();
  comp = compatibles;
  while (comp - compatibles < len) {
    g_ptr_array_add (parts, (gpointer) g_strdup (comp));
    comp = strchr (comp, 0);
    comp++;
  }
  g_ptr_array_add (parts, NULL);
  return (GStrv) g_ptr_array_steal (parts, NULL);
#else
  env_compatibles = get_env_compatibles();
  if (env_compatibles)
    return env_compatibles;

  g_set_error_literal (err, G_IO_ERROR, G_IO_ERROR_NOT_SUPPORTED,
                       "Not supported on this platform");
  return NULL;
#endif
}
