/*
 * Copyright (C) 2022 The Phosh Developers
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Author: Guido Günther <agx@sigxcpu.org>
 */

#include "gm-error.h"
#include "gm-rect.h"
#include "gm-svg-path.h"

#include <math.h>

#if !GLIB_CHECK_VERSION(2, 74, 0)
# define G_REGEX_DEFAULT 0
# define G_REGEX_MATCH_DEFAULT 0
#endif

struct bbox {
  int x1, x2, y1, y2;

  int cx, cy;
};


struct fbbox {
  double x1, x2, y1, y2;
};


static void
swap (double *a, double *b)
{
  double tmp = *a;

  *a = *b;
  *b = tmp;
}

static double
bbox_quad_deriv (double start, double control, double end)
{
  double t;

  /* control point between start and end, nothing to do */
  if ((control > start &&  control < end) || (control > end && control < start))
    return start;

  t = ((start - control) / (start - (2 * control) + end));
  return start * ((1 - t) * (1 - t)) + 2 * control * (1 - t) * t + end * t * t;
}

/*
 * See https://pomax.github.io/bezierinfo/ ,
 *     http://pomax.nihongoresources.com/pages/bezier/
 */
static struct fbbox
bbox_quadratic_bezier (double x1, double y1,
                       double x2, double y2,
                       double x3, double y3)
{
  struct fbbox bbox = { 0.0 };
  double x, y;

  bbox.x1 = fmin (x1, x3);
  bbox.y1 = fmin (y1, y3);
  bbox.x2 = fmax (x1, x3);
  bbox.y2 = fmax (y1, y3);

  x = bbox_quad_deriv (x1, x2, x3);
  if (x < bbox.x1)
    bbox.x1 = x;
  else if (x > bbox.x2)
    bbox.x2 = x;

  y = bbox_quad_deriv (y1, y2, y3);
  if (y < bbox.y1)
    bbox.y1 = y;
  else if (y > bbox.y2)
    bbox.y2 = y;

  return bbox;
}


static inline double
bbox_arc_get_angle (double bx, double by)
{
  return fmod (2 * M_PI + (by > 0.0 ? 1.0 : -1.0) *
               acos ( bx / sqrt (bx * bx + by * by) ), 2 * M_PI);
}

/* See https://fridrich.blogspot.com/2011/06/bounding-box-of-svg-elliptical-arc.html */
static struct fbbox
bbox_arc (double x1, double y1,
          double rx, double ry, double phi, gboolean large_arc, gboolean sweep,
          double x2, double y2)
{
  struct fbbox bbox;
  gboolean other_arc;
  const double x1prime = cos (phi)*(x1 - x2)/2 + sin (phi)*(y1 - y2) / 2;
  const double y1prime = -sin (phi)*(x1 - x2)/2 + cos (phi)*(y1 - y2) / 2;
  double txmin, txmax, tymin, tymax, cx, cy, radicant, angle1, angle2;
  double cxprime = 0.0;
  double cyprime = 0.0;

  if (rx < 0.0)
    rx *= -1.0;
  if (ry < 0.0)
    ry *= -1.0;

  if (G_APPROX_VALUE (rx, 0.0, DBL_EPSILON) ||
      G_APPROX_VALUE (ry, 0.0, DBL_EPSILON)) {
    bbox.x1 = (x1 < x2 ? x1 : x2);
    bbox.x2 = (x1 > x2 ? x1 : x2);
    bbox.y1 = (y1 < y2 ? y1 : y2);
    bbox.y2 = (y1 > y2 ? y1 : y2);
    return bbox;
  }

  radicant = (rx*rx*ry*ry - rx*rx*y1prime*y1prime - ry*ry*x1prime*x1prime);
  radicant /= (rx*rx*y1prime*y1prime + ry*ry*x1prime*x1prime);
  if (radicant < 0.0) {
    double ratio = rx/ry;
    double rad = y1prime * y1prime + x1prime * x1prime / (ratio * ratio);
    if (rad < 0.0) {
      bbox.x1 = (x1 < x2 ? x1 : x2);
      bbox.x2 = (x1 > x2 ? x1 : x2);
      bbox.y1 = (y1 < y2 ? y1 : y2);
      bbox.y2 = (y1 > y2 ? y1 : y2);
      return bbox;
    }
    ry =sqrt (rad);
    rx = ratio * ry;
  } else {
    double factor = ((large_arc == sweep) ? -1.0 : 1.0) * sqrt (radicant);

    cxprime = factor * rx* y1prime / ry;
    cyprime = -factor * ry * x1prime / rx;
  }

  cx = cxprime * cos (phi) - cyprime * sin (phi) + (x1 + x2) / 2;
  cy = cxprime * sin (phi) + cyprime * cos (phi) + (y1 + y2) / 2;

  if (G_APPROX_VALUE (phi, 0.0, DBL_EPSILON) ||
      G_APPROX_VALUE (phi, M_PI, DBL_EPSILON)) {
    bbox.x1 = cx - rx;
    txmin = bbox_arc_get_angle (-rx, 0);
    bbox.x2 = cx + rx;
    txmax = bbox_arc_get_angle (rx, 0);
    bbox.y1 = cy - ry;
    tymin = bbox_arc_get_angle (0, -ry);
    bbox.y2 = cy + ry;
    tymax = bbox_arc_get_angle (0, ry);
  } else if (G_APPROX_VALUE (phi, M_PI / 2.0, DBL_EPSILON) ||
             G_APPROX_VALUE (phi, 3.0 * M_PI / 2.0, DBL_EPSILON)) {
    bbox.x1 = cx - ry;
    txmin = bbox_arc_get_angle (-ry, 0);
    bbox.x2 = cx + ry;
    txmax = bbox_arc_get_angle (ry, 0);
    bbox.y1 = cy - rx;
    tymin = bbox_arc_get_angle (0, -rx);
    bbox.y2 = cy + rx;
    tymax = bbox_arc_get_angle (0, rx);
  } else {
    double tmp_x, tmp_y;
    txmin = -atan (ry*tan (phi)/rx);
    txmax = M_PI - atan (ry*tan (phi)/rx);
    bbox.x1 = cx + rx * cos (txmin) * cos (phi) - ry * sin (txmin) * sin (phi);
    bbox.x2 = cx + rx * cos (txmax) * cos (phi) - ry * sin (txmax) * sin (phi);
    if (bbox.x1 > bbox.x2) {
      swap (&bbox.x1, &bbox.x2);
      swap (&txmin, &txmax);
    }
    tmp_y = cy + rx * cos (txmin) * sin (phi) + ry * sin (txmin) * cos (phi);
    txmin = bbox_arc_get_angle (bbox.x1 - cx, tmp_y - cy);
    tmp_y = cy + rx * cos (txmax) * sin (phi) + ry * sin (txmax) * cos (phi);
    txmax = bbox_arc_get_angle (bbox.x2 - cx, tmp_y - cy);

    tymin = atan (ry / (tan (phi) * rx));
    tymax = atan (ry / (tan (phi) * rx)) + M_PI;
    bbox.y1 = cy + rx * cos (tymin) * sin (phi) + ry * sin (tymin) * cos (phi);
    bbox.y2 = cy + rx * cos (tymax) * sin (phi) + ry * sin (tymax) * cos (phi);
    if (bbox.y1 > bbox.y2) {
      swap (&bbox.y1, &bbox.y2);
      swap (&tymin, &tymax);
    }
    tmp_x = cx + rx * cos (tymin) * cos (phi) - ry * sin (tymin) * sin (phi);
    tymin = bbox_arc_get_angle (tmp_x - cx, bbox.y1 - cy);
    tmp_x = cx + rx * cos (tymax) * cos (phi) - ry * sin (tymax) * sin (phi);
    tymax = bbox_arc_get_angle (tmp_x - cx, bbox.y2 - cy);
  }

  angle1 = bbox_arc_get_angle (x1 - cx, y1 - cy);
  angle2 = bbox_arc_get_angle (x2 - cx, y2 - cy);

  if (!sweep)
    swap (&angle1, &angle2);

  other_arc = FALSE;
  if (angle1 > angle2) {
    swap (&angle1, &angle2);
    other_arc = TRUE;
  }

  if ((!other_arc && (angle1 > txmin || angle2 < txmin)) ||
      (other_arc && !(angle1 > txmin || angle2 < txmin)))
    bbox.x1 = x1 < x2 ? x1 : x2;
  if ((!other_arc && (angle1 > txmax || angle2 < txmax)) ||
      (other_arc && !(angle1 > txmax || angle2 < txmax)))
    bbox.x2 = x1 > x2 ? x1 : x2;
  if ((!other_arc && (angle1 > tymin || angle2 < tymin)) ||
      (other_arc && !(angle1 > tymin || angle2 < tymin)))
    bbox.y1 = y1 < y2 ? y1 : y2;
  if ((!other_arc && (angle1 > tymax || angle2 < tymax)) ||
      (other_arc && !(angle1 > tymax || angle2 < tymax)))
    bbox.y2 = y1 > y2 ? y1 : y2;

  return bbox;
}


static gboolean
parse_int (const char *str, int *number, GError **err)
{
  gint64 nl;
  char *endptr;

  if (str == NULL) {
    g_set_error_literal (err, GM_ERROR, GM_ERROR_PARSING_FAILED, "empty string is not a number");
    return FALSE;
  }

  nl  = g_ascii_strtoll (str, &endptr, 10);
  if (nl == 0 && str == endptr) {
    g_set_error (err, GM_ERROR, GM_ERROR_PARSING_FAILED, "'%s' not a number", str);
    return FALSE;
  }

  if (ABS (nl) > G_MAXINT) {
    g_set_error (err, GM_ERROR, GM_ERROR_PARSING_FAILED, "'%s' too large", str);
    return FALSE;
  }

  *number = nl;
  return TRUE;
}


static void
extend_bbox_by_point (struct bbox *bbox, int x, int y)
{
  if (x < bbox->x1)
    bbox->x1 = x;
  if (x > bbox->x2)
    bbox->x2 = x;

  if (y < bbox->y1)
    bbox->y1 = y;
  if (y > bbox->y2)
    bbox->y2 = y;

  bbox->cx = x;
  bbox->cy = y;
}


static void
extend_bbox_by_frect (struct bbox *bbox, struct fbbox *rect)
{
  if (rect->x1 < bbox->x1)
    bbox->x1 = rect->x1;
  if (rect->x2 > bbox->x2)
    bbox->x2 = rect->x2;
  if (rect->y1 < bbox->y1)
    bbox->y1 = rect->y1;
  if (rect->y2 > bbox->y2)
    bbox->y2 = rect->y2;
}


static gboolean
is_command (char c)
{
  switch (c) {
  case 'A': /* arc */
  case 'a':
  case 'C': /* cubic bezier */
  case 'c':
  case 'H': /* horizonal line */
  case 'h':
  case 'L': /* line */
  case 'l':
  case 'M': /* move to */
  case 'm':
  case 'Q': /* quadratic bezier (TBD) */
  case 'q':
  case 'S': /* shortcut cubic bezier (TBD) */
  case 's':
  case 'T': /* shortcut quadratic bezier (TBD) */
  case 't':
  case 'V': /* vertical line */
  case 'v':
  case 'Z': /* close path */
    return TRUE;
  default:
    return FALSE;
  }
}


static char *
normalize_path (const char *path)
{
  GString *canon = g_string_new ("");
  char *norm;

  for (int i = 0; path[i] != '\0'; i++) {
    if (path[i] == ',' || path[i] == '\n' || path[i] == '\t') {
      /* Comma works like whitespace */
      g_string_append_c (canon, ' ');
    } else if (is_command (path[i])) {
      /* Make sure there's whitespace after each command */
      g_string_append_c (canon, path[i]);
      /* Make sure there's whitespace after each command */
      g_string_append_c (canon, ' ');
    } else {
      g_string_append_c (canon, path[i]);
    }
  }

  norm = g_string_free (canon, FALSE);

  return g_strstrip (norm);
}


/**
 * gm_svg_path_get_bounding_box:
 * @path: An SVG path
 * @x1: The lower x coordinate
 * @x2: The upper x coordinate
 * @y1: The lower y coordinate
 * @y2: The upper x coordinate
 * @err: Return location for an error
 *
 * Returns the bounding box of an SVG path. As this is meant for
 * display cutouts we operate on integer (whole pixel) values.  When
 * parsing fails, `FALSE` is returned and `error` contains the error
 * information.
 *
 * Returns: `TRUE` when parsing was successful, `FALSE` otherwise.
 *
 * See https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths for path syntax introduction.
 *
 * Since: 0.0.1
 */
gboolean
gm_svg_path_get_bounding_box (const char *path, int *x1, int *x2, int *y1, int *y2, GError **err)
{
  g_auto (GStrv) parts = NULL;
  g_autoptr (GRegex) whitespace = NULL;
  g_autofree char *norm = NULL;
  struct bbox bbox = { G_MAXINT, 0, G_MAXINT, 0 };

  g_return_val_if_fail (err == NULL || *err == NULL, FALSE);

  norm = normalize_path (path);

  whitespace = g_regex_new (" +", G_REGEX_DEFAULT, G_REGEX_MATCH_DEFAULT, err);
  g_return_val_if_fail (whitespace, FALSE);
  parts = g_regex_split (whitespace, norm, G_REGEX_MATCH_DEFAULT);

  for (int i = 0; parts[i] != NULL; i++) {
    const char *cmd = parts[i];
    int x, y, dx, dy;
    gboolean rel = FALSE;

    switch (cmd[0]) {
    case 'M': /* x,y */
    case 'L':
      i++;
      if (parse_int (parts[i], &x, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &y, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, x, y);
      break;
    case 'm': /* dx,dy */
    case 'l':
      i++;
      if (parse_int (parts[i], &dx, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &dy, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, bbox.cx + dx, bbox.cy + dy);
      break;
    case 'V': /* y */
      i++;
      if (parse_int (parts[i], &y, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, bbox.cx, y);
      break;
    case 'v': /* dy */
      i++;
      if (parse_int (parts[i], &dy, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, bbox.cx, bbox.cy + dy);
      break;
    case 'H': /* x */
      i++;
      if (parse_int (parts[i], &x, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, x, bbox.cy);
      break;
    case 'h': /* dx */
      i++;
      if (parse_int (parts[i], &dx, err) == FALSE)
        return FALSE;
      extend_bbox_by_point (&bbox, bbox.cx + dx, bbox.cy);
      break;
    case 'a':
      rel = TRUE;
      G_GNUC_FALLTHROUGH;
    case 'A': { /* rx ry x-axis-rotation large-arc-flag sweep-flag x y */
      int rx, ry, xrot;
      gboolean large, sweep;
      struct fbbox fbox;

      i++;
      if (parse_int (parts[i], &rx, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &ry, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &xrot, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &large, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &sweep, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &x, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &y, err) == FALSE)
        return FALSE;

      if (rel) {
        x = bbox.cx + x;
        y = bbox.cy + y;
      }

      fbox = bbox_arc (bbox.cx, bbox.cy,
                       rx, ry, xrot, !!large, !!sweep,
                       x, y);
      extend_bbox_by_frect (&bbox, &fbox);
      bbox.cx = x;
      bbox.cy = y;
      break;
    }
    case 'q':
      rel = TRUE;
      G_GNUC_FALLTHROUGH;
    case 'Q': {
      int cx, cy;
      struct fbbox fbox;

      /* control point */
      i++;
      if (parse_int (parts[i], &cx, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &cy, err) == FALSE)
        return FALSE;
      /* end */
      i++;
      if (parse_int (parts[i], &x, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &y, err) == FALSE)
        return FALSE;

      if (rel) {
        cx = bbox.cx + cx;
        cy = bbox.cy + cy;
        x  = bbox.cx + x;
        y  = bbox.cy + y;
      }

      fbox = bbox_quadratic_bezier (bbox.cx, bbox.cy, cx, cy, x, y);
      extend_bbox_by_frect (&bbox, &fbox);
      bbox.cx = x;
      bbox.cy = y;
      break;
    }
    case 'c':
      rel = TRUE;
      G_GNUC_FALLTHROUGH;
    case 'C': {
      int cx1, cy1, cx2, cy2;

      /* control point 1 */
      i++;
      if (parse_int (parts[i], &cx1, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &cy1, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &cx2, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &cy2, err) == FALSE)
        return FALSE;
      /* end */
      i++;
      if (parse_int (parts[i], &x, err) == FALSE)
        return FALSE;
      i++;
      if (parse_int (parts[i], &y, err) == FALSE)
        return FALSE;

      if (rel) {
        cx1 = bbox.cx + cx1;
        cy1 = bbox.cy + cy1;
        cx2 = bbox.cx + cx2;
        cy2 = bbox.cy + cy2;
        x  = bbox.cx + x;
        y  = bbox.cy + y;
      }

#if 0
      struct fbbox fbox;
      /* TODO properly calculate minimal bbox using derivate */
      fbox = bbox_cubic_bezier (bbox.cx, bbox.cy, cx, cy, x, y);
      extend_bbox_by_frect (&bbox, &fbox);
#else
      extend_bbox_by_point (&bbox, x, y);
      extend_bbox_by_point (&bbox, cx1, cy1);
      extend_bbox_by_point (&bbox, cx2, cy2);
#endif
      bbox.cx = x;
      bbox.cy = y;
      break;
    }
    case 'Z':
      break;
    default:
      g_set_error (err, GM_ERROR, GM_ERROR_PARSING_FAILED, "Unknown command '%s'", cmd);
      return FALSE;
    }
  }

  *x1 = bbox.x1;
  *x2 = bbox.x2;
  *y1 = bbox.y1;
  *y2 = bbox.y2;
  return TRUE;
}
