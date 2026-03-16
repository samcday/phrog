%bcond_without check
%bcond_without vendor
%global cargo_install_lib 0
%if %{with vendor}
%global _cargo_generate_buildrequires 0
%endif

Name:           phrog
Version:        0.53.0_rc4
Release:        %autorelease
Summary:        Mobile-friendly greeter for greetd
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

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
BuildRequires:  meson
BuildRequires:  pam-devel
BuildRequires:  pkgconfig(appstream)
BuildRequires:  pkgconfig(libecal-2.0)
BuildRequires:  pkgconfig(libedataserver-1.2)
BuildRequires:  pkgconfig(fribidi)
BuildRequires:  pkgconfig(gcr-3)
BuildRequires:  pkgconfig(glib-2.0)
BuildRequires:  pkgconfig(gio-2.0)
BuildRequires:  pkgconfig(gio-unix-2.0)
BuildRequires:  pkgconfig(gmobile)
BuildRequires:  pkgconfig(gmodule-no-export-2.0)
BuildRequires:  pkgconfig(gnome-bluetooth-3.0)
BuildRequires:  pkgconfig(gnome-desktop-3.0)
BuildRequires:  pkgconfig(gobject-2.0)
BuildRequires:  pkgconfig(gsettings-desktop-schemas)
BuildRequires:  pkgconfig(gtk+-3.0)
BuildRequires:  pkgconfig(gtk+-wayland-3.0)
BuildRequires:  pkgconfig(gudev-1.0)
BuildRequires:  pkgconfig(libfeedback-0.0)
BuildRequires:  pkgconfig(libhandy-1)
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
BuildRequires:  pkgconfig(evince-document-3.0)
BuildRequires:  pkgconfig(evince-view-3.0)
BuildRequires:  phoc

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
%{__cargo} vendor --locked --versioned-dirs vendor
%cargo_prep -v vendor
%else
%cargo_prep
%generate_buildrequires
%cargo_generate_buildrequires
%endif

%build
export PHROG_LIBPHOSH_BUILD_INTERNAL=always
export PHOSH_SRC=$PWD/phosh
%cargo_build
%cargo_vendor_manifest
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.service -t %{buildroot}%{_userunitdir}/
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.target -t %{buildroot}%{_userunitdir}/
%{__install} -Dpm 0644 data/mobi.phosh.phrog.gschema.xml -t %{buildroot}%{_datadir}/glib-2.0/schemas/
%{__install} -Dpm 0644 data/phrog.session -t %{buildroot}%{_datadir}/gnome-session/sessions/
%{__install} -Dpm 0644 data/mobi.phosh.Phrog.desktop -t %{buildroot}%{_datadir}/applications/
%{__install} -Dpm 0644 dist/fedora/greetd-config.toml -t %{buildroot}%{_sysconfdir}/phrog/
%{__install} -Dpm 0644 dist/fedora/phrog.service -t %{buildroot}%{_unitdir}/
%{__install} -Dpm 0644 data/systemd-session.conf -T %{buildroot}%{_userunitdir}/gnome-session@phrog.target.d/session.conf
%{__install} -Dpm 0755 data/phrog-greetd-session -t %{buildroot}%{_libexecdir}/
%{__install} -d %{buildroot}%{_datadir}/phrog/autostart
%{__install} -d %{buildroot}%{_sysconfdir}/phrog/autostart
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
cat > test.sh <<HERE
#!/bin/bash
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
