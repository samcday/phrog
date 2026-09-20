// Meson emits the private dependencies needed when embedding libphosh. Promote
// only this library's fields; pkg-config can then resolve its dependencies as
// shared libraries without pulling their private implementation libraries in.
pub fn embedding_metadata(metadata: &str) -> String {
    let mut shared_metadata = String::new();
    let mut requires = Vec::new();
    let mut libs = Vec::new();
    for line in metadata.lines() {
        if let Some(value) = line
            .strip_prefix("Requires:")
            .or_else(|| line.strip_prefix("Requires.private:"))
        {
            requires.push(value.trim());
        } else if let Some(value) = line
            .strip_prefix("Libs:")
            .or_else(|| line.strip_prefix("Libs.private:"))
        {
            libs.push(value.trim());
        } else {
            shared_metadata.push_str(line);
            shared_metadata.push('\n');
        }
    }
    shared_metadata.push_str(&format!(
        "Requires: {}\nLibs: {}\n",
        requires.join(", "),
        libs.join(" ")
    ));
    shared_metadata
}
