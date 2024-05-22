%global cargo_install_lib 0

Name:           phrog
Version:        0.1.11
Release:        %autorelease
Summary:        Greetd-compatible greeter for mobile phones
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24

Requires:       greetd >= 0.6
Provides:       greetd-greeter = 0.6
Provides:       greetd-%{name} = %{version}

# we are building our own private copy of libphosh
#BuildRequires:	gcc
#BuildRequires:	meson
#BuildRequires:	pam-devel
#BuildRequires:	phoc
#BuildRequires:	pkgconfig(libecal-2.0) >= 3.33.1
#BuildRequires:	pkgconfig(libedataserver-1.2) >= 3.33.1
#BuildRequires:	pkgconfig(fribidi)
#BuildRequires:	pkgconfig(gcr-3) >= 3.7.5
#BuildRequires:	pkgconfig(glib-2.0) >= 2.76
#BuildRequires:	pkgconfig(gio-2.0) >= 2.76
#BuildRequires:	pkgconfig(gio-unix-2.0) >= 2.76
#BuildRequires:	pkgconfig(gmobile) >= 0.1.0
#BuildRequires:	pkgconfig(gmodule-no-export-2.0) >= 2.76
#BuildRequires:	pkgconfig(gnome-desktop-3.0) >= 3.26
#BuildRequires:	pkgconfig(gobject-2.0) >= 2.76
#BuildRequires:	pkgconfig(gsettings-desktop-schemas) >= 42
#BuildRequires:	pkgconfig(gtk+-3.0) >= 3.22
#BuildRequires:	pkgconfig(gtk+-wayland-3.0) >= 3.22
#BuildRequires:	pkgconfig(gudev-1.0)
#BuildRequires:	pkgconfig(libfeedback-0.0) >= 0.2.0
#BuildRequires:	pkgconfig(libhandy-1) >= 1.1.90
#BuildRequires:	pkgconfig(libnm) >= 1.14
#BuildRequires:	pkgconfig(polkit-agent-1) >= 0.105
#BuildRequires:	pkgconfig(libsoup-3.0) >= 3.0
#BuildRequires:	pkgconfig(libsystemd) >= 241
#BuildRequires:	pkgconfig(libsecret-1)
#BuildRequires:	pkgconfig(upower-glib) >= 0.99.1
#BuildRequires:	pkgconfig(wayland-client) >= 1.14
#BuildRequires:	pkgconfig(wayland-protocols) >= 1.12
#BuildRequires:	pkgconfig(evince-document-3.0)
#BuildRequires:  pkgconfig(evince-view-3.0)
#BuildRequires:	pkgconfig(gtk4) >= 4.0
#BuildRequires:	pkgconfig(libadwaita-1) >= 1.2
#BuildRequires:	pkgconfig(alsa)
#BuildRequires:	pkgconfig(libpulse) >= 12.99.3
#BuildRequires:	pkgconfig(libpulse-mainloop-glib)
#BuildRequires:	pkgconfig(libcallaudio-0.1)

Requires:	phosh

Requires: phosh-devel
BuildRequires: phosh-devel

%description
%{summary}.

%prep
%autosetup -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%doc README.md
%{_bindir}/phrog

%changelog
%autochangelog
