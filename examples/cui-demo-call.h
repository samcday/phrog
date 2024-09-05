#pragma once

#include <cui-call.h>

G_BEGIN_DECLS

#define CUI_TYPE_DEMO_CALL (cui_demo_call_get_type())

G_DECLARE_FINAL_TYPE (CuiDemoCall, cui_demo_call, CUI, DEMO_CALL, GObject)

CuiDemoCall *cui_demo_call_new (gboolean inbound);
void         cui_demo_call_set_encrypted (CuiDemoCall *self, gboolean encrypted);

G_END_DECLS
