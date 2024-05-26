/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-timeout.h"

#include <fcntl.h>
#include <unistd.h>
#include <sys/timerfd.h>

#if defined(__DragonFly__) || defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__)
/* https://github.com/jiixyj/epoll-shim/issues/45 */
#undef CLOCK_BOOTTIME
#define CLOCK_BOOTTIME CLOCK_MONOTONIC
#endif

typedef struct _GmTimeoutOnce {
  GSource  source;
  int      fd;
  gpointer tag;
  gulong   timeout_ms;
  gboolean armed;
} GmTimeoutOnce;


static gboolean
gm_timeout_once_prepare (GSource *source, gint *timeout)
{
  GmTimeoutOnce *timer = (GmTimeoutOnce *)source;
  struct itimerspec time_spec = { 0 };
  int ret;

  if (timer->fd == -1)
    return FALSE;

  if (timer->armed)
    return FALSE;

  time_spec.it_value.tv_sec = timer->timeout_ms / 1000;
  time_spec.it_value.tv_nsec = (timer->timeout_ms % 1000) * 1000;

  ret = timerfd_settime (timer->fd, 0 /* flags */, &time_spec, NULL);
  if (ret)
    g_warning ("Failed to set up timer: %s", strerror (ret));

  g_debug ("Prepared %p[%s] for %ld seconds",
	   source,
	   g_source_get_name (source)?: "(null)",
	   timer->timeout_ms / 1000);
  timer->armed = TRUE;
  /* Never wake up the source due to a timeout */
  *timeout = -1;
  return FALSE;
}


static gboolean
gm_timeout_once_dispatch (GSource     *source,
                          GSourceFunc  callback,
                          void        *data)
{
  if (!callback) {
    g_warning ("Timeout source dispatched without callback. "
               "You must call g_source_set_callback().");
    return G_SOURCE_REMOVE;
  }

  g_debug ("Dispatching %p[%s]", source, g_source_get_name (source)?: "(null)");
  callback (data);

  return G_SOURCE_REMOVE;
}


static void
gm_timeout_once_finalize (GSource *source)
{
  GmTimeoutOnce *timer = (GmTimeoutOnce *) source;

  close (timer->fd);
  timer->fd = -1;
  timer->armed = FALSE;

  g_source_remove_unix_fd (source, timer->tag);
  timer->tag = NULL;

  g_debug ("Finalize %p[%s]", source, g_source_get_name (source)?: "(null)");
}


static GSourceFuncs gm_timeout_once_source_funcs = {
  gm_timeout_once_prepare,
  NULL, /* check */
  gm_timeout_once_dispatch,
  gm_timeout_once_finalize,
};


static GSource *
gm_timeout_source_once_new (gulong timeout_ms)
{
  int fdf, fsf;
  GmTimeoutOnce *timer = (GmTimeoutOnce *) g_source_new (&gm_timeout_once_source_funcs,
                                                         sizeof (GmTimeoutOnce));

  timer->timeout_ms = timeout_ms;
#if GLIB_CHECK_VERSION(2, 70, 0)
  g_source_set_static_name ((GSource *)timer, "[gm] boottime timeout source");
#endif
  timer->fd = timerfd_create (CLOCK_BOOTTIME, 0);
  if (timer->fd == -1)
    return (GSource*)timer;

  fdf = fcntl (timer->fd, F_GETFD) | FD_CLOEXEC;
  fcntl (timer->fd, F_SETFD, fdf);
  fsf = fcntl (timer->fd, F_GETFL) | O_NONBLOCK;
  fcntl (timer->fd, F_SETFL, fsf);

  timer->tag = g_source_add_unix_fd (&timer->source, timer->fd, G_IO_IN | G_IO_ERR);
  return (GSource*)timer;
}


/**
 * gm_timeout_add_seconds_once_full: (rename-to gm_timeout_add_seconds_once)
 * @priority: the priority of the timeout source. Typically this will be in
 *   the range between %G_PRIORITY_DEFAULT and %G_PRIORITY_HIGH.
 * @seconds: the timeout in seconds
 * @function: function to call
 * @data: data to pass to @function
 * @notify: (nullable): function to call when the timeout is removed, or %NULL
 *
 * Sets a function to be called after a timeout with priority @priority.
 * Correctly calculates the timeout even when the system is suspended in between.
 *
 * This internally creates a main loop source using
 * g_timeout_source_new_seconds() and attaches it to the main loop context
 * using g_source_attach().
 *
 * The timeout given is in terms of `CLOCK_BOOTTIME` time, it hence is also
 * correct across suspend and resume. If that doesn't matter use
 * `g_timeout_add_seconds_full` instead.
 *
 * Note that glib's `g_timeout_add_seconds()` doesn't take system
 * suspend/resume into account: https://gitlab.gnome.org/GNOME/glib/-/issues/2739
 *
 * Returns: the ID (greater than 0) of the event source.
 *
 * Since: 0.0.1
 **/
guint
gm_timeout_add_seconds_once_full (gint            priority,
                                  gulong          seconds,
                                  GSourceOnceFunc function,
                                  gpointer        data,
                                  GDestroyNotify  notify)
{
  g_autoptr (GSource) source = NULL;
  guint id;

  g_return_val_if_fail (function != NULL, 0);

  source = gm_timeout_source_once_new (1000L * seconds);

  if (priority != G_PRIORITY_DEFAULT)
    g_source_set_priority (source, priority);

  g_source_set_callback (source, (GSourceFunc)function, data, notify);
  id = g_source_attach (source, NULL);

  return id;
}

/**
 * gm_timeout_add_seconds_once:
 * @seconds: the timeout in seconds
 * @function: function to call
 * @data: data to pass to @function
 *
 * Sets a function to be called after a timeout with the default
 * priority, %G_PRIORITY_DEFAULT. Correctly calculates the timeout
 * even when the system is suspended in between.
 *
 * This internally creates a main loop source using
 * g_timeout_source_new_seconds() and attaches it to the main loop context
 * using g_source_attach().
 *
 * The timeout given is in terms of `CLOCK_BOOTTIME` time, it hence is also
 * correct across suspend and resume. If that doesn't matter use
 * `g_timeout_add_seconds` instead.
 *
 * Note that glib's `g_timeout_add_seconds()` doesn't take system
 * suspend/resume into account: https://gitlab.gnome.org/GNOME/glib/-/issues/2739
 *
 * Returns: the ID (greater than 0) of the event source.
 *
 * Since: 0.0.1
 **/
guint
gm_timeout_add_seconds_once (int             seconds,
                             GSourceOnceFunc function,
                             gpointer        data)
{
  g_return_val_if_fail (function != NULL, 0);

  return gm_timeout_add_seconds_once_full (G_PRIORITY_DEFAULT, seconds, function, data, NULL);
}
