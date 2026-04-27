use std::error::Error;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{self, Command};

use clap::{Parser, Subcommand};
use semver::Version;
use toml_edit::{value, DocumentMut};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const GREETD_CONFIG_TEMPLATE: &str = include_str!("../../data/greetd-config.toml.in");
const VERSION_USAGE: &str = "Use X.Y.Z or X.Y.Z-rc.N.";

#[derive(Debug, PartialEq, Eq)]
struct ReleaseVersion {
    cargo: String,
    package: String,
    debian: String,
    is_rc: bool,
}

#[derive(Parser)]
#[command(about = "phrog development tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bump version across all packaging files.
    Bump {
        /// Version to bump to (X.Y.Z or X.Y.Z-rc.N).
        version: String,
    },
    /// Generate packaging data.
    DistData {
        /// File name for the generated greetd config under target/dist-data.
        file_name: String,
        /// VT to run greetd on.
        #[arg(long)]
        greetd_vt: u8,
        /// User greetd should use for phrog.
        #[arg(long)]
        greetd_user: String,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Commands::Bump { version } => bump(&version),
        Commands::DistData {
            file_name,
            greetd_vt,
            greetd_user,
        } => dist_data(&file_name, greetd_vt, &greetd_user),
    }
}

fn bump(version: &str) -> Result<()> {
    let version = parse_version(version)?;
    let root = project_root()?;

    println!("Bumping phrog to {}", version.cargo);

    update_cargo_toml(&root.join("Cargo.toml"), &version.cargo)?;
    replace_line(
        &root.join("phrog.spec"),
        |line| line.starts_with("Version:"),
        &format!("Version:        {}", version.package),
    )?;
    replace_line(
        &root.join("APKBUILD"),
        |line| line.starts_with("pkgver=") && line.ends_with("_git"),
        &format!("pkgver={}_git", version.package),
    )?;

    run_command(
        &root,
        "dch",
        &[
            "--newversion",
            &format!("{}-1", version.debian),
            "--distribution",
            "unstable",
            "--urgency",
            "medium",
            &format!("Release {}", version.cargo),
        ],
        &[("DEBEMAIL", "phrog@beep.boop"), ("DEBFULLNAME", "Phrogbot")],
    )?;

    if !version.is_rc {
        update_readme_demo(&root.join("README.md"), &version.cargo)?;
    }

    run_command(&root, "cargo", &["generate-lockfile"], &[])?;

    println!("Done. Files updated:");
    println!("  Cargo.toml");
    println!("  phrog.spec");
    println!("  APKBUILD");
    println!("  debian/changelog");
    if !version.is_rc {
        println!("  README.md");
    }
    println!("  Cargo.lock");
    println!();

    Ok(())
}

fn dist_data(file_name: &str, greetd_vt: u8, greetd_user: &str) -> Result<()> {
    let root = project_root()?;
    let out_path = dist_data_path(&root, file_name)?;
    let out_dir = out_path
        .parent()
        .ok_or_else(|| format!("output path '{}' has no parent", out_path.display()))?;

    fs::create_dir_all(&out_dir)?;
    fs::write(&out_path, render_greetd_config(greetd_vt, greetd_user))?;

    println!("Generated {}", out_path.display());

    Ok(())
}

fn parse_version(version: &str) -> Result<ReleaseVersion> {
    let parsed = Version::parse(version)
        .map_err(|error| format!("unsupported version '{version}': {error}. {VERSION_USAGE}"))?;

    if !parsed.build.is_empty() {
        return Err(format!("unsupported version '{version}'. {VERSION_USAGE}").into());
    }

    if parsed.pre.is_empty() {
        return Ok(ReleaseVersion {
            cargo: parsed.to_string(),
            package: parsed.to_string(),
            debian: parsed.to_string(),
            is_rc: false,
        });
    }

    let rc = parsed
        .pre
        .as_str()
        .strip_prefix("rc.")
        .filter(|rc| rc.bytes().all(|byte| byte.is_ascii_digit()))
        .ok_or_else(|| format!("unsupported version '{version}'. {VERSION_USAGE}"))?;
    let base = format!("{}.{}.{}", parsed.major, parsed.minor, parsed.patch);

    Ok(ReleaseVersion {
        cargo: parsed.to_string(),
        package: format!("{base}_rc{rc}"),
        debian: format!("{base}~rc{rc}"),
        is_rc: true,
    })
}

fn render_greetd_config(greetd_vt: u8, greetd_user: &str) -> String {
    GREETD_CONFIG_TEMPLATE
        .replace("@VT@", &greetd_vt.to_string())
        .replace("@USER@", greetd_user)
}

fn dist_data_path(root: &Path, file_name: &str) -> Result<PathBuf> {
    let path = Path::new(file_name);
    let mut components = path.components();

    match (components.next(), components.next()) {
        (Some(Component::Normal(_)), None) => Ok(root.join("target/dist-data").join(path)),
        _ => {
            Err(format!("dist-data file name must not contain path separators: {file_name}").into())
        }
    }
}

fn project_root() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask manifest directory has no parent".into())
}

fn update_cargo_toml(path: &Path, cargo_version: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let updated = update_cargo_toml_text(&text, cargo_version)?;

    fs::write(path, updated)?;
    Ok(())
}

fn update_cargo_toml_text(text: &str, cargo_version: &str) -> Result<String> {
    let mut document = text.parse::<DocumentMut>()?;
    let package = document["package"]
        .as_table_mut()
        .ok_or("package section not found in Cargo.toml")?;

    if !package.contains_key("version") {
        return Err("package version not found in Cargo.toml".into());
    }

    package["version"] = value(cargo_version);

    Ok(document.to_string())
}

fn replace_line(path: &Path, predicate: impl Fn(&str) -> bool, replacement: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let mut replaced = false;
    let mut updated = String::with_capacity(text.len());

    for line in text.lines() {
        if !replaced && predicate(line) {
            updated.push_str(replacement);
            replaced = true;
        } else {
            updated.push_str(line);
        }
        updated.push('\n');
    }

    if !replaced {
        return Err(format!("matching line not found in {}", path.display()).into());
    }

    fs::write(path, updated)?;
    Ok(())
}

fn update_readme_demo(path: &Path, version: &str) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let updated = update_readme_demo_text(&text, version)?;

    fs::write(path, updated)?;
    Ok(())
}

fn update_readme_demo_text(text: &str, version: &str) -> Result<String> {
    let marker = "releases/download/";
    let suffix = "/demo.webp";
    let mut updated = String::with_capacity(text.len() + version.len());
    let mut copied_until = 0;
    let mut search_from = 0;
    let mut replaced = false;

    while let Some(marker_offset) = text[search_from..].find(marker) {
        let marker_start = search_from + marker_offset;
        let version_start = marker_start + marker.len();
        let Some(version_len) = text[version_start..].find('/') else {
            break;
        };
        let version_end = version_start + version_len;

        if version_end > version_start && text[version_end..].starts_with(suffix) {
            updated.push_str(&text[copied_until..version_start]);
            updated.push_str(version);
            copied_until = version_end;
            search_from = version_end + suffix.len();
            replaced = true;
        } else {
            search_from = version_start;
        }
    }

    if !replaced {
        return Err("README demo video URL not found".into());
    }

    updated.push_str(&text[copied_until..]);

    Ok(updated)
}

fn run_command(root: &Path, program: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<()> {
    let mut command = Command::new(program);
    command
        .current_dir(root)
        .args(args)
        .envs(envs.iter().copied());

    let status = command
        .status()
        .map_err(|error| format!("failed to run {program}: {error}"))?;

    if !status.success() {
        return Err(format!("{program} failed with {status}").into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stable_version() {
        assert_eq!(
            parse_version("1.2.3").unwrap(),
            ReleaseVersion {
                cargo: "1.2.3".into(),
                package: "1.2.3".into(),
                debian: "1.2.3".into(),
                is_rc: false,
            }
        );
    }

    #[test]
    fn parses_rc_version() {
        assert_eq!(
            parse_version("1.2.3-rc.4").unwrap(),
            ReleaseVersion {
                cargo: "1.2.3-rc.4".into(),
                package: "1.2.3_rc4".into(),
                debian: "1.2.3~rc4".into(),
                is_rc: true,
            }
        );
    }

    #[test]
    fn rejects_unsupported_prerelease_version() {
        let error = parse_version("1.2.3-beta.1").unwrap_err().to_string();
        assert!(error.contains(VERSION_USAGE));
    }

    #[test]
    fn rejects_build_metadata_version() {
        let error = parse_version("1.2.3+build.1").unwrap_err().to_string();
        assert!(error.contains(VERSION_USAGE));
    }

    #[test]
    fn renders_greetd_config() {
        assert_eq!(
            render_greetd_config(7, "_greetd"),
            "[terminal]\nvt = 7\n\n[default_session]\ncommand = \"/usr/libexec/phrog-greetd-session\"\nuser = \"_greetd\"\n\n# The session to be used on boot\n#[initial_session]\n#command = \"systemd-cat phosh-session\"\n#user = \"username\"\n"
        );
    }

    #[test]
    fn builds_dist_data_path() {
        assert_eq!(
            dist_data_path(Path::new("/tmp/phrog"), "phrog.toml").unwrap(),
            PathBuf::from("/tmp/phrog/target/dist-data/phrog.toml")
        );
    }

    #[test]
    fn rejects_nested_dist_data_path() {
        assert_eq!(
            dist_data_path(Path::new("/tmp/phrog"), "debian/phrog.toml")
                .unwrap_err()
                .to_string(),
            "dist-data file name must not contain path separators: debian/phrog.toml"
        );
    }

    #[test]
    fn updates_cargo_toml_package_version_only() {
        let text = concat!(
            "[package]\n",
            "name = \"phrog\"\n",
            "version = \"1.2.3\"\n\n",
            "[dependencies]\n",
            "clap = { version = \"4\" }\n"
        );

        assert_eq!(
            update_cargo_toml_text(text, "2.0.0").unwrap(),
            concat!(
                "[package]\n",
                "name = \"phrog\"\n",
                "version = \"2.0.0\"\n\n",
                "[dependencies]\n",
                "clap = { version = \"4\" }\n"
            )
        );
    }

    #[test]
    fn updates_readme_demo_download_url() {
        let text =
            "<img src=\"https://github.com/samcday/phrog/releases/download/1.2.3/demo.webp\">";

        assert_eq!(
            update_readme_demo_text(text, "2.0.0").unwrap(),
            "<img src=\"https://github.com/samcday/phrog/releases/download/2.0.0/demo.webp\">"
        );
    }

    #[test]
    fn updates_all_readme_demo_download_urls() {
        let text = "releases/download/1.2.3/demo.webp releases/download/1.2.3/demo.webp";

        assert_eq!(
            update_readme_demo_text(text, "2.0.0").unwrap(),
            "releases/download/2.0.0/demo.webp releases/download/2.0.0/demo.webp"
        );
    }

    #[test]
    fn ignores_other_readme_release_download_urls() {
        let text = "releases/download/1.2.3/source.tar.gz releases/download/1.2.3/demo.webp";

        assert_eq!(
            update_readme_demo_text(text, "2.0.0").unwrap(),
            "releases/download/1.2.3/source.tar.gz releases/download/2.0.0/demo.webp"
        );
    }
}
