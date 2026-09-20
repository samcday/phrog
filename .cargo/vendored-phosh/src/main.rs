use std::{env, ffi::OsStr, os::unix::process::CommandExt, path::Path, process::Command};

fn main() {
    let config = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../vendor.toml")
        .canonicalize()
        .expect("vendored Phosh configuration is missing");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
    let mut args = env::args_os().skip(1);
    match args.next() {
        // cargo-fmt has its own argument parser and rejects --config. It does
        // not compile dependencies; Cargo can consume the configuration first.
        Some(subcommand) if subcommand == OsStr::new("fmt") => {
            command.arg("--config").arg(&config).arg(subcommand);
        }
        Some(subcommand) => {
            // Pass the option through to commands such as cargo-clippy, which
            // launch another Cargo process and need to forward the config.
            command.arg(subcommand).arg("--config").arg(&config);
        }
        None => {
            command.arg("--config").arg(&config).arg("--help");
        }
    }
    let error = command.args(args).exec();
    eprintln!("failed to execute Cargo: {error}");
    std::process::exit(1);
}
