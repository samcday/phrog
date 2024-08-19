#pragma once

#include <call-ui.h>

G_BEGIN_DECLS

#define CUI_TYPE_DUMMY_CALL (cui_dummy_call_get_type())

G_DECLARE_FINAL_TYPE (CuiDummyCall, cui_dummy_call, CUI, DUMMY_CALL, GObject)

CuiDummyCall     *cui_dummy_call_new               (void);
void              cui_dummy_call_set_id            (CuiDummyCall *self,
                                                    const char   *id);
void              cui_dummy_call_set_display_name  (CuiDummyCall *self,
                                                    const char   *display_name);
void              cui_dummy_call_set_can_dtmf      (CuiDummyCall *self,
                                                    gboolean      enabled);
void              cui_dummy_call_set_encrypted     (CuiDummyCall *self,
                                                    gboolean      enabled);
void              cui_dummy_call_set_state         (CuiDummyCall *self,
                                                    CuiCallState  state);

G_END_DECLS
