%global cargo_install_lib 0
%bcond static 0

Name:           phrog
Version:        0.9.0
Release:        %autorelease
Summary:        Greetd-compatible greeter for mobile phones
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24
BuildRequires:  pkgconfig(libphosh-0)

%if %{with static}
BuildRequires:	gcc
BuildRequires:	meson
BuildRequires:	pam-devel
BuildRequires:	pkgconfig(libecal-2.0) >= 3.33.1
BuildRequires:	pkgconfig(libedataserver-1.2) >= 3.33.1
BuildRequires:	pkgconfig(fribidi)
BuildRequires:	pkgconfig(gcr-3) >= 3.7.5
BuildRequires:	pkgconfig(glib-2.0) >= 2.76
BuildRequires:	pkgconfig(gio-2.0) >= 2.76
BuildRequires:	pkgconfig(gio-unix-2.0) >= 2.76
BuildRequires:	pkgconfig(gmobile) >= 0.1.0
BuildRequires:	pkgconfig(gnome-bluetooth-3.0) >= 46.0
BuildRequires:	pkgconfig(gnome-desktop-3.0) >= 3.26
BuildRequires:	pkgconfig(gobject-2.0) >= 2.76
BuildRequires:	pkgconfig(gsettings-desktop-schemas) >= 42
BuildRequires:	pkgconfig(gtk+-3.0) >= 3.24.36
BuildRequires:	pkgconfig(gtk+-wayland-3.0) >= 3.22
BuildRequires:	pkgconfig(gudev-1.0)
BuildRequires:	pkgconfig(libfeedback-0.0) >= 0.4.0
BuildRequires:	pkgconfig(libhandy-1) >= 1.1.90
BuildRequires:	pkgconfig(libnm) >= 1.14
BuildRequires:	pkgconfig(polkit-agent-1) >= 0.105
BuildRequires:	pkgconfig(libsoup-3.0) >= 3.0
BuildRequires:	pkgconfig(libsystemd) >= 241
BuildRequires:	pkgconfig(libsecret-1)
BuildRequires:	pkgconfig(upower-glib) >= 0.99.1
BuildRequires:	pkgconfig(wayland-client) >= 1.14
BuildRequires:	pkgconfig(wayland-protocols) >= 1.12
BuildRequires:	pkgconfig(gtk4) >= 4.8.3
BuildRequires:	pkgconfig(libadwaita-1)
BuildRequires:	pkgconfig(evince-document-3.0)
BuildRequires:	pkgconfig(evince-view-3.0)
BuildRequires:	pkgconfig(alsa)
BuildRequires:	pkgconfig(libpulse) >= 12.99.3
BuildRequires:	pkgconfig(libpulse-mainloop-glib)
BuildRequires:	pkgconfig(libcallaudio-0.1)
BuildRequires:	/usr/bin/xvfb-run
BuildRequires:	/usr/bin/xauth
BuildRequires:	dbus-daemon
BuildRequires:	desktop-file-utils
BuildRequires:	systemd-rpm-macros

# Statically linking phosh requires these. No idea why.
BuildRequires:	jbigkit-devel
BuildRequires:	pkgconfig(com_err)
BuildRequires:	pkgconfig(krb5-gssapi)
BuildRequires:	pkgconfig(Lerc)
BuildRequires:	libunistring-devel
%endif

Requires:       greetd >= 0.6
Requires:       libphosh >= 0.41

# for dbus-launch
Requires:       dbus-x11

Provides:       greetd-greeter = 0.6
Provides:       greetd-%{name} = %{version}

%description
%{summary}.

%prep
%autosetup -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build %{?with_static:-f static}
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
install -d %{buildroot}%{_datadir}/glib-2.0/schemas/
%{__install} -Dpm 0644 resources/mobi.phosh.phrog.gschema.xml %{buildroot}%{_datadir}/glib-2.0/schemas/
%cargo_install %{?with_static:-f static}

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%doc README.md
%{_datadir}/glib-2.0/schemas/*
%{_bindir}/phrog

%changelog
%autochangelog
