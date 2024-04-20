fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/phrog.gresources.xml",
        "phrog.gresource",
    );
}
