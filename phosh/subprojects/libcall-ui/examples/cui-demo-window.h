#pragma once

#include <call-ui.h>
#include <handy.h>

G_BEGIN_DECLS

#define CUI_TYPE_DEMO_WINDOW (cui_demo_window_get_type())

G_DECLARE_FINAL_TYPE (CuiDemoWindow, cui_demo_window, CUI, DEMO_WINDOW, HdyApplicationWindow)

CuiDemoWindow *cui_demo_window_new (GtkApplication *application);

G_END_DECLS
