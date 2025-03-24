# Tests are disabled until rust-wayland-protocols-misc is available
# https://bugzilla.redhat.com/show_bug.cgi?id=2344240
%bcond check 0
%global cargo_install_lib 0

Name:           phrog
Version:        0.46.0
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

Requires:       squeekboard
Requires:       gnome-session
Requires:       greetd
Requires:       phoc

%description
Phrog uses Phosh and greetd to provide a graphical login manager.

%prep
%autosetup -p1
%cargo_prep
# tests need a writable XDG_RUNTIME_DIR
mkdir -p /tmp/runtime-dir
chmod 0700 /tmp/runtime-dir

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
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

%if %{with check}
%check
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

%changelog
%autochangelog
