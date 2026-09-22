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

# Nuke gcr rather than fixing Gck and don't care about GnomeBluetooth
# Drop doc:format (see https://github.com/gtk-rs/gir/issues/1642)
xmlstarlet ed -L \
	-d '///_:include[@name="Gcr"]' \
	-d '///_:include[@name="GnomeBluetooth"]' \
	Phosh-0.gir

# GtkPlain is not introspected yet (WIP GTK custom-surface work): g-i emits
# unnamed <type> elements for the parent_instance field, which trip up gir.
# Point them at Gtk.Widget so instance structs keep a workable layout.
xmlstarlet ed -L \
	--var plain_parent '//_:field[@name="parent_instance"]/_:type[@c:type="GtkPlain" and not(@name)]' \
	-i '$plain_parent' -t attr -n 'name' -v 'Gtk.Widget' \
	-u '$plain_parent/@c:type' -v 'GtkWidget' \
	Phosh-0.gir
