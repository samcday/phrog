%global cargo_install_lib 0
%global phosh_ver 0.44

Name:           phrog
Version:        0.44.0
Release:        %autorelease
Summary:        Greetd-compatible greeter for mobile phones
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24
BuildRequires:  pkgconfig(libphosh-%{phosh_ver})

Requires:       greetd >= 0.6
Requires:       libphosh >= %{phosh_ver}

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
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
install -d %{buildroot}%{_datadir}/glib-2.0/schemas/
%{__install} -Dpm 0644 resources/mobi.phosh.phrog.gschema.xml %{buildroot}%{_datadir}/glib-2.0/schemas/
%cargo_install

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
