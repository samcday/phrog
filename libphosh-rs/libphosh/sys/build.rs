#[cfg(not(docsrs))]
use std::process;

mod build_version;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new()
        .add_build_internal("libphosh_0", |_, version| {
            // TODO: locate a meson dir.
            // Either one has already been prepared for us somewhere, or we use
            // the local meson.build
            system_deps::Library::from_internal_pkg_config(format!("/usr/local/lib64/phosh/{}/pkgconfig", version), "libphosh-0", version)
        })
        .probe()
    {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
