use crate::common::SupervisedChild;
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};
use zbus::zvariant::ObjectPath;

pub fn system_dbus(tmpdir: &Path) -> SupervisedChild {
    let config_path = tmpdir.join("system-dbus.xml");
    let sock_path = tmpdir.join("system.sock");
    let dbus_path = format!("unix:path={}", sock_path.display());
    std::fs::write(
        &config_path,
        format!(
            r#"
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
    "#,
            dbus_path
        ),
    )
    .expect("failed to write system dbus config");

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

    SupervisedChild::new("dbus-daemon", child)
}

struct AccountsFixture {
    num_users: Option<u32>,
}
struct UserFixture {
    name: String,
    username: String,
    icon_file: String,
}

#[zbus::interface(name = "org.freedesktop.Accounts")]
impl AccountsFixture {
    async fn list_cached_users(&self) -> Vec<ObjectPath> {
        let mut users = vec![
            ObjectPath::from_static_str_unchecked("/org/freedesktop/Accounts/phoshi"),
            ObjectPath::from_static_str_unchecked("/org/freedesktop/Accounts/agx"),
            ObjectPath::from_static_str_unchecked("/org/freedesktop/Accounts/sam"),
        ];
        if let Some(num_users) = self.num_users {
            users.truncate(num_users as _);
        }
        users
    }
}

impl UserFixture {
    fn new(name: &str, username: &str, icon_file: &str) -> Self {
        Self {
            name: name.into(),
            username: username.into(),
            icon_file: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/")
                .join(icon_file)
                .display()
                .to_string(),
        }
    }
}

#[zbus::interface(name = "org.freedesktop.Accounts.User")]
impl UserFixture {
    #[zbus(property)]
    async fn real_name(&self) -> &str {
        &self.name
    }
    #[zbus(property)]
    async fn user_name(&self) -> &str {
        &self.username
    }
    #[zbus(property)]
    async fn icon_file(&self) -> &str {
        &self.icon_file
    }
}

pub async fn run_accounts_fixture(num_users: Option<u32>) -> anyhow::Result<zbus::Connection> {
    let connection = zbus::Connection::system()
        .await
        .context("failed to connect to system bus")?;
    connection
        .object_server()
        .at("/org/freedesktop/Accounts", AccountsFixture { num_users })
        .await
        .context("failed to serve org.freedesktop.Accounts")?;
    connection
        .object_server()
        .at(
            "/org/freedesktop/Accounts/agx",
            UserFixture::new("Guido", "agx", "guido.png"),
        )
        .await
        .context("failed to serve org.freedesktop.Accounts.User")?;
    connection
        .object_server()
        .at(
            "/org/freedesktop/Accounts/phoshi",
            UserFixture::new("Phoshi", "phoshi", "phoshi.png"),
        )
        .await
        .context("failed to serve org.freedesktop.Accounts.User")?;
    connection
        .object_server()
        .at(
            "/org/freedesktop/Accounts/sam",
            UserFixture::new("Sam", "samcday", "samcday.jpeg"),
        )
        .await
        .context("failed to serve org.freedesktop.Accounts.User")?;
    connection
        .request_name("org.freedesktop.Accounts")
        .await
        .context("failed to request name")?;
    Ok(connection)
}
