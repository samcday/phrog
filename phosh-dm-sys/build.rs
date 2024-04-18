use std::path::PathBuf;

mod build_version;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    // if let Err(s) = system_deps::Config::new().probe() {
    //     println!("cargo:warning={s}");
    //     std::process::exit(1);
    // }

    let mut build_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    build_path = build_path.join("build");
    let build_path = build_path.to_str().unwrap();

    let static_lib: PathBuf = [build_path, "src"].iter().collect();
    println!("cargo:rustc-link-lib=static=phosh-tool");
    println!("cargo:rustc-link-lib=static=phosh");
    println!("cargo:rustc-link-search=native={}", static_lib.to_str().unwrap());
    meson::build("../phosh", build_path);

    let libcall_static_lib: PathBuf = [build_path, "subprojects/libcall-ui/src"].iter().collect();
    println!("cargo:rustc-link-search=native={}", libcall_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=call-ui");

    let libgvc_static_lib: PathBuf = [build_path, "subprojects/gvc"].iter().collect();
    println!("cargo:rustc-link-search=native={}", libgvc_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=gvc");


    let libgmobile_static_lib: PathBuf = [build_path, "subprojects/gmobile/src"].iter().collect();
    println!("cargo:rustc-link-search=native={}", libgmobile_static_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=gmobile");

    // TODO: get phosh to barf a pkg-config and then we shouldn't need this
    println!("cargo:rustc-link-lib=wayland-client");
    println!("cargo:rustc-link-lib=nm");
    println!("cargo:rustc-link-lib=secret-1");
    // println!("cargo:rustc-link-lib=callaudio");
    println!("cargo:rustc-link-lib=handy-1");
    println!("cargo:rustc-link-lib=pulse");
    println!("cargo:rustc-link-lib=pulse-mainloop-glib");
    println!("cargo:rustc-link-lib=fribidi");
    // println!("cargo:rustc-link-lib=gnome-desktop");
    // println!("cargo:rustc-link-lib=gsettings-desktop-schemas");
    // println!("cargo:rustc-link-lib=gtk+-wayland");
    println!("cargo:rustc-link-lib=gudev-1.0");
    println!("cargo:rustc-link-lib=gmodule-2.0");
    println!("cargo:rustc-link-lib=systemd");
    println!("cargo:rustc-link-lib=polkit-agent-1");
    println!("cargo:rustc-link-lib=polkit-gobject-1");
    println!("cargo:rustc-link-lib=pam");
    println!("cargo:rustc-link-lib=callaudio-0.1");
    println!("cargo:rustc-link-lib=gcr-base-3");
    println!("cargo:rustc-link-lib=gcr-ui-3");
    println!("cargo:rustc-link-lib=feedback-0.0");
    println!("cargo:rustc-link-lib=gnome-desktop-3");
    println!("cargo:rustc-link-lib=soup-3.0");
    println!("cargo:rustc-link-lib=upower-glib");
    println!("cargo:rustc-link-lib=json-glib-1.0");

    // lib-0.1, gio-2.0 >= 2.76, gtk+-3.0 >= 3.22,
    // lib-1 >= 1.2, gobject-2.0 >= 2.76, gio-unix-2.0 >= 2.76,
    // glib-2.0 >= 2.76, -2.0 >= 2.76,
    //  >=  0.1.0, -3.0 >= 3.26,  >= 42, -3.0 >= 3.22, -1.0, libfeedback-0.0 >=  0.2.0,
    //  polkit-agent-1 >=  0.105,  upower-glib >= 0.99.1,
}
