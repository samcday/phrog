#!/bin/bash
set -x -e

# MM's doc-version trips up ./gir
xmlstarlet ed -L \
	-d '///_:doc-version' \
	NM-1.0.gir

# NM uses uint32 instead of guint32 in one place:
xmlstarlet ed -L \
	-i '//_:interface[@name="Connection"]/_:method[@name="diff"]//_:parameter[@name="out_settings"]//_:type[@c:type="uint32"]' -t 'attr' -n 'name' -v 'guint32' \
	NM-1.0.gir

# Nuke gcr rather than fixing Gck
xmlstarlet ed -L \
	-d '///_:include[@name="Gcr"]' \
	Phosh-0.gir

# Avoid having to use gvc gir
xmlstarlet ed -L \
	-i '///_:type[@c:type="GvcMixerControl*"]' -t 'attr' -n 'name' -v 'gpointer' \
	Phosh-0.gir

# AuthAgent uses non introspectable PolkitAgentListenerClass
xmlstarlet ed -L \
	   -d '///_:class[@name="PolkitAuthAgent"]' \
	   -d '///_:record[@name="PolkitAuthAgentClass"]' \
	   -d '///_:function[@name="polkit_authentication_agent_register"]' \
	Phosh-0.gir

# SystemPrompt uses Gcr.SystemPrompt which needs Gck which has lots of introspection bugs
xmlstarlet ed -L \
	   -d '///_:class[@name="SystemPrompt"]' \
	   -d '///_:record[@name="SystemPromptClass"]' \
	   -d '///_:function[@name="system_prompter_register"]' \
	Phosh-0.gir

# off_t is not a glib type
xmlstarlet ed -L \
	   -d '///_:function[@name="create_shm_file"]' \
	Phosh-0.gir

# GnomeBluetooth is complicated at present. No direct need for it in bindings (atm).
xmlstarlet ed -L \
	   -d '///_:class[@name="BtDeviceRow"]' \
	   -d '///_:class[@name="BtManager"]' \
	   -d '///_:method[@c:identifier="phosh_shell_get_bt_manager"]' \
	   -d '///_:record[@name="BtManagerClass"]' \
	   -d '///_:include[@name="GnomeBluetooth"]' \
	Phosh-0.gir
