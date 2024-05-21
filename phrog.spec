Name:           phrog
Version:        0.1.0
Release:        %autorelease
Summary:        Greetd-compatible greeter for mobile phones
License:        GPL-3.0-only
URL:            https://github.com/samcday/phrog
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust-packaging >= 26

Requires:       greetd >= 0.6
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
%cargo_install
rm %{_bindir}/fakegreet

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
