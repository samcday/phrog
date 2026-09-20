use std::{fs, io, path::Path};

const SUBPROJECTS: [&str; 2] = ["gvc", "libcall-ui"];

// Meson's downloader writes into the source tree. Stage a private copy so a
// build can use a read-only checkout and concurrent Cargo profiles stay isolated.
pub fn stage(source: &Path, output: &Path) -> io::Result<()> {
    let next = output.with_extension("next");
    let previous = output.with_extension("previous");
    // Recover an interrupted swap before discarding any incomplete new tree.
    if previous.exists() {
        if output.exists() {
            fs::remove_dir_all(&previous)?;
        } else {
            fs::rename(&previous, output)?;
        }
    }
    if next.exists() {
        fs::remove_dir_all(&next)?;
    }
    copy_tree(source, &next)?;
    for name in SUBPROJECTS {
        let relative = Path::new("subprojects").join(name);
        let prepared = source.join(&relative);
        let cached = output.join(&relative);
        let destination = next.join(&relative);
        let wrap = format!("subprojects/{name}.wrap");
        if prepared.is_dir() {
            copy_tree(&prepared, &destination)?;
        } else if cached.is_dir() && fs::read(source.join(&wrap))? == fs::read(output.join(&wrap))?
        {
            copy_tree(&cached, &destination)?;
        }
    }
    if output.exists() {
        fs::rename(output, &previous)?;
    }
    if let Err(error) = fs::rename(next, output) {
        if previous.exists() {
            fs::rename(&previous, output)?;
        }
        return Err(error);
    }
    if previous.exists() {
        fs::remove_dir_all(previous)?;
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" || name == "_build" {
            continue;
        }
        // These are copied from prepared inputs or recovered from the cache
        // separately, after checking that the wrap revision has not changed.
        if source.file_name().is_some_and(|name| name == "subprojects")
            && SUBPROJECTS.iter().any(|project| name == *project)
        {
            continue;
        }
        let target = destination.join(&name);
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            std::os::unix::fs::symlink(fs::read_link(entry.path())?, target)?;
        } else if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
