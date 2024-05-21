Name:		phrog
Version:	0.0.0
Release:	1%{?dist}
Summary:	Greetd-compatible greeter for mobile phones
License:	GPLv3+
URL:		https://github.com/samcday/phrog
Source0:	https://github.com/samcday/phrog/-/archive/v%{version}/%{name}-v%{version}.tar.gz

# Tests failing
ExcludeArch:	i686

BuildRequires:	gcc
BuildRequires:	meson
BuildRequires:	pam-devel
BuildRequires:	rust-packaging
BuildRequires:	pkgconfig(libecal-2.0) >= 3.33.1
BuildRequires:	pkgconfig(libedataserver-1.2) >= 3.33.1
BuildRequires:	pkgconfig(fribidi)
BuildRequires:	pkgconfig(gcr-3) >= 3.7.5
BuildRequires:	pkgconfig(glib-2.0) >= 2.76
BuildRequires:	pkgconfig(gio-2.0) >= 2.76
BuildRequires:	pkgconfig(gio-unix-2.0) >= 2.76
BuildRequires:	pkgconfig(gmobile) >= 0.1.0
BuildRequires:	pkgconfig(gmodule-no-export-2.0) >= 2.76
BuildRequires:	pkgconfig(gnome-desktop-3.0) >= 3.26
BuildRequires:	pkgconfig(gobject-2.0) >= 2.76
BuildRequires:	pkgconfig(gsettings-desktop-schemas) >= 42
BuildRequires:	pkgconfig(gtk+-3.0) >= 3.22
BuildRequires:	pkgconfig(gtk+-wayland-3.0) >= 3.22
BuildRequires:	pkgconfig(gudev-1.0)
BuildRequires:	pkgconfig(libfeedback-0.0) >= 0.2.0
BuildRequires:	pkgconfig(libhandy-1) >= 1.1.90
BuildRequires:	pkgconfig(libnm) >= 1.14
BuildRequires:	pkgconfig(polkit-agent-1) >= 0.105
BuildRequires:	pkgconfig(libsoup-3.0) >= 3.0
BuildRequires:	pkgconfig(libsystemd) >= 241
BuildRequires:	pkgconfig(libsecret-1)
BuildRequires:	pkgconfig(upower-glib) >= 0.99.1
BuildRequires:	pkgconfig(wayland-client) >= 1.14
BuildRequires:	pkgconfig(wayland-protocols) >= 1.12
# Plugin dependencies:
BuildRequires:	pkgconfig(evince-document-3.0)
BuildRequires:  pkgconfig(evince-view-3.0)
BuildRequires:	pkgconfig(gtk4) >= 4.0
BuildRequires:	pkgconfig(libadwaita-1) >= 1.2
# GVC dependencies:
BuildRequires:	pkgconfig(alsa)
BuildRequires:	pkgconfig(libpulse) >= 12.99.3
BuildRequires:	pkgconfig(libpulse-mainloop-glib)
# libcallui dependencies:
BuildRequires:	pkgconfig(libcallaudio-0.1)

BuildRequires:	desktop-file-utils
BuildRequires:	systemd-rpm-macros

# Test dependencies
BuildRequires:	dbus-daemon
BuildRequires:  /usr/bin/xvfb-run

Requires:	phoc >= 0.25.0
Requires:	iio-sensor-proxy
Requires:	gnome-session
Requires:	gnome-shell
Requires:	lato-fonts
Requires:	hicolor-icon-theme

Recommends:	squeekboard >= 1.21.0

%description
Greetd-compatible greeter for mobile phones


%prep
%autosetup -p1 -n %{name}-v%{version}
%cargo_prep


%build
%meson -Dphoc_tests=disabled -Dsystemd=true
%meson_build


%install
%meson_install
%find_lang %{name}

%{__install} -Dpm 0644 data/phosh.service %{buildroot}%{_unitdir}/phosh.service


%check



%files -f %{name}.lang
%{_bindir}/phosh-session
%{_libexecdir}/phosh
%{_libexecdir}/phosh-calendar-server
%{_datadir}/applications/sm.puri.Phosh.desktop
%{_datadir}/glib-2.0/schemas/*
%{_datadir}/gnome-session/sessions/phosh.session
%{_datadir}/wayland-sessions/phosh.desktop
%{_datadir}/phosh
%{_sysconfdir}/pam.d/phosh
%{_unitdir}/phosh.service
%{_userunitdir}/gnome-session@phosh.target.d/session.conf
%{_userunitdir}/sm.puri.Phosh.service
%{_userunitdir}/sm.puri.Phosh.target
%{_datadir}/applications/sm.puri.OSK0.desktop
%{_datadir}/xdg-desktop-portal/portals/phosh.portal
%{_datadir}/xdg-desktop-portal/phosh-portals.conf
%{_libdir}/phosh
%{_datadir}/icons/hicolor/symbolic/apps/sm.puri.Phosh-symbolic.svg
%{_datadir}/dbus-1/services/sm.puri.Phosh.CalendarServer.service

%doc README.md
%license COPYING


%changelog

* Tue May 21 2024 Sam Day <me@samcday.com> - 0.0.0

- Initial packaging
