%bcond_without check
%bcond_without vendor
%global cargo_install_lib 0
%if %{with vendor}
%global _cargo_generate_buildrequires 0
%endif

# gettext-sys's build.rs falls back to compiling its bundled gettext when
# this is unset, and that bundled build is broken on Fedora 43+. Force the
# system libintl path; gettext-devel provides the headers.
%global gettext_sys_env export GETTEXT_SYSTEM=1

Name:           phrog
Version:        0.53.0
Release:        %autorelease
Summary:        Mobile-friendly greeter for greetd
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source0:        %{url}/archive/%{version}/%{name}-%{version}.tar.gz
%if %{with vendor}
Source1:        %{name}-vendor.tar.gz
%endif

ExcludeArch:    %{ix86}

BuildRequires:  cargo-rpm-macros >= 24
# for dbus-run-session in %check
BuildRequires:	dbus-daemon
# for xvfb-run in %check
BuildRequires:  xorg-x11-server-Xvfb
# first-run test uses foot
BuildRequires:  foot
BuildRequires:  gcc
BuildRequires:  git-core
BuildRequires:  gnome-shell-common
BuildRequires:  gnome-settings-daemon
BuildRequires:  meson
BuildRequires:  mutter-common
BuildRequires:  pam-devel
BuildRequires:  pkgconfig(appstream)
BuildRequires:  pkgconfig(libecal-2.0)
BuildRequires:  pkgconfig(libedataserver-1.2)
BuildRequires:  pkgconfig(fribidi)
BuildRequires:  pkgconfig(gcr-4)
BuildRequires:  pkgconfig(glib-2.0)
BuildRequires:  pkgconfig(gio-2.0)
BuildRequires:  pkgconfig(gio-unix-2.0)
BuildRequires:  pkgconfig(gmobile)
BuildRequires:  pkgconfig(gmodule-no-export-2.0)
BuildRequires:  pkgconfig(gnome-bluetooth-3.0)
BuildRequires:  pkgconfig(gnome-desktop-4)
BuildRequires:  pkgconfig(gobject-2.0)
BuildRequires:  pkgconfig(gsettings-desktop-schemas)
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(gudev-1.0)
BuildRequires:  pkgconfig(libfeedback-0.0)
%if %{with vendor}
# link closure for the vendor tarball build, where
# %generate_buildrequires is unavailable
BuildRequires:  pkgconfig(cairo)
BuildRequires:  pkgconfig(cairo-gobject)
BuildRequires:  pkgconfig(gdk-pixbuf-2.0)
BuildRequires:  pkgconfig(graphene-gobject-1.0)
BuildRequires:  pkgconfig(pango)
%endif
BuildRequires:  pkgconfig(libnm)
BuildRequires:  pkgconfig(polkit-agent-1)
BuildRequires:  pkgconfig(libsoup-3.0)
BuildRequires:  pkgconfig(libsystemd)
BuildRequires:  pkgconfig(mm-glib)
BuildRequires:  pkgconfig(libsecret-1)
BuildRequires:  libunistring-devel
BuildRequires:  pkgconfig(upower-glib)
BuildRequires:  pkgconfig(wayland-client)
BuildRequires:  pkgconfig(wayland-protocols)
BuildRequires:  pkgconfig(alsa)
BuildRequires:  pkgconfig(libpulse)
BuildRequires:  pkgconfig(libpulse-mainloop-glib)
BuildRequires:  pkgconfig(libcallaudio-0.1)
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  pkgconfig(gcr-4)
BuildRequires:  pkgconfig(gnome-desktop-4)
BuildRequires:  phoc
BuildRequires:  gettext-devel

Requires:       accountsservice
Requires:       gnome-session
Requires:       greetd
Requires:       phoc
Requires:       phosh-osk = 1.0

%description
Phrog uses Phosh and greetd to provide a graphical login manager.

%prep
%autosetup -p1

# tests need a writable XDG_RUNTIME_DIR
mkdir -p /tmp/runtime-dir
chmod 0700 /tmp/runtime-dir

%if %{with vendor}
tar -xf %{SOURCE1}
%cargo_prep -v vendor
%else
%cargo_prep
%generate_buildrequires
%cargo_generate_buildrequires
%endif

%build
export PHROG_LIBPHOSH_BUILD_INTERNAL=always
export PHOSH_SRC=$PWD/phosh
%{gettext_sys_env}
%cargo_build
%{__cargo} run --offline --quiet --package xtask -- dist-data greetd-config.toml --greetd-vt 1 --greetd-user greetd
%cargo_vendor_manifest
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.service -t %{buildroot}%{_userunitdir}/
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.target -t %{buildroot}%{_userunitdir}/
%{__install} -Dpm 0644 data/mobi.phosh.phrog.gschema.xml -t %{buildroot}%{_datadir}/glib-2.0/schemas/
%{__install} -Dpm 0644 data/00_mobi.phosh.Phrog.gschema.override -t %{buildroot}%{_datadir}/glib-2.0/schemas/
%{__install} -Dpm 0644 data/phrog.session -t %{buildroot}%{_datadir}/gnome-session/sessions/
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.desktop -t %{buildroot}%{_datadir}/applications/
%{__install} -Dpm 0644 target/dist-data/greetd-config.toml -t %{buildroot}%{_sysconfdir}/phrog/
%{__install} -Dpm 0644 dist/fedora/phrog.service -t %{buildroot}%{_unitdir}/
%{__install} -Dpm 0644 data/systemd-session.conf -T %{buildroot}%{_userunitdir}/gnome-session@phrog.target.d/session.conf
%{__install} -Dpm 0755 data/phrog-greetd-session -t %{buildroot}%{_libexecdir}/
%{__install} -d %{buildroot}%{_datadir}/phrog/autostart
%{__install} -d %{buildroot}%{_sysconfdir}/phrog/autostart
%{gettext_sys_env}
%cargo_install
if ldd %{buildroot}%{_bindir}/phrog | grep -q libphosh; then
  echo "phrog is dynamically linked against libphosh"
  exit 1
fi

%if %{with check}
%check
export PHROG_LIBPHOSH_BUILD_INTERNAL=always
export PHOSH_SRC=$PWD/phosh
export G_MESSAGES_DEBUG=all
export XDG_RUNTIME_DIR=/tmp/runtime-dir
export LD_LIBRARY_PATH=%{_libdir}/pulseaudio${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
cat > test.sh <<HERE
#!/bin/bash
%{gettext_sys_env}
%cargo_test
HERE
chmod +x test.sh
dbus-run-session xvfb-run -a -s -noreset phoc -S -E ./test.sh
%endif

%files
%license LICENSE
%doc README.md
%{_bindir}/phrog
%{_datadir}/applications/mobi.phosh.Phrog.desktop
%{_datadir}/glib-2.0/schemas/mobi.phosh.phrog.gschema.xml
%{_datadir}/glib-2.0/schemas/00_mobi.phosh.Phrog.gschema.override
%{_datadir}/gnome-session/sessions/phrog.session
%{_datadir}/phrog
%{_datadir}/phrog/autostart
%{_libexecdir}/phrog-greetd-session
%{_sysconfdir}/phrog
%{_sysconfdir}/phrog/autostart
%config(noreplace) %{_sysconfdir}/phrog/greetd-config.toml
%{_unitdir}/phrog.service
%{_userunitdir}/gnome-session@phrog.target.d/session.conf
%{_userunitdir}/mobi.phosh.Phrog.service
%{_userunitdir}/mobi.phosh.Phrog.target

%changelog
%autochangelog
