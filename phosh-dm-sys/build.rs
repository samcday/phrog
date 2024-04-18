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

    println!("cargo:rustc-link-lib=libphosh");
    println!("cargo:rustc-link-search=native={}", build_path);
    meson::build("../../phosh", build_path);
}
