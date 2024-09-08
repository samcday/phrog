use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::time::{Duration, Instant};
use anyhow::Context;
use zbus::zvariant::ObjectPath;
use crate::common::SupervisedChild;

pub fn system_dbus(tmpdir: &Path) -> SupervisedChild {
    let config_path = tmpdir.join("system-dbus.xml");
    let sock_path = tmpdir.join("system.sock");
    let dbus_path = format!("unix:path={}", sock_path.display());
    std::fs::write(&config_path, format!(r#"
    <!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-Bus Bus Configuration 1.0//EN"
     "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
    <busconfig>
      <type>system</type>
      <keep_umask/>
      <listen>{}</listen>
      <policy context="default">
        <allow send_destination="*" eavesdrop="true"/>
        <allow eavesdrop="true"/>
        <allow own="*"/>
      </policy>
    </busconfig>
    "#, dbus_path)).expect("failed to write system dbus config");

    std::env::set_var("DBUS_SYSTEM_BUS_ADDRESS", dbus_path);
    let child = std::process::Command::new("dbus-daemon")
        .arg(format!("--config-file={}", config_path.to_str().unwrap()))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to launch dbus-daemon");

    let start = Instant::now();
    while !sock_path.exists() {
        if start.elapsed() > Duration::from_secs(5) {
            panic!("dbus-daemon failed to launch");
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    SupervisedChild(child)
}

struct AccountsFixture {}
struct UserFixture {}

#[zbus::interface(name = "org.freedesktop.Accounts")]
impl AccountsFixture {
    async fn list_cached_users(&self) -> Vec<ObjectPath> {
        vec![ObjectPath::from_static_str_unchecked("/org/freedesktop/Accounts/1")]
    }
}

#[zbus::interface(name = "org.freedesktop.Accounts.User")]
impl UserFixture {
    #[zbus(property)]
    async fn real_name(&self) -> &str {
        "Phoshi"
    }
    #[zbus(property)]
    async fn user_name(&self) -> &str {
        "phoshi"
    }
    #[zbus(property)]
    async fn icon_file(&self) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/phoshi.png").display().to_string()
    }
}

pub async fn run_accounts_fixture() -> anyhow::Result<zbus::Connection> {
    let connection = zbus::Connection::system().await.context("failed to connect to system bus")?;
    connection
        .object_server()
        .at("/org/freedesktop/Accounts", AccountsFixture {})
        .await.context("failed to serve org.freedesktop.Accounts")?;
    connection
        .object_server()
        .at("/org/freedesktop/Accounts/1", UserFixture {})
        .await.context("failed to serve org.freedesktop.Accounts.User")?;
    connection
        .request_name("org.freedesktop.Accounts")
        .await.context("failed to request name")?;
    Ok(connection)
}
