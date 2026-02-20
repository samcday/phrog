use crate::common::SupervisedChild;
use anyhow::Context;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use zbus::zvariant::ObjectPath;

pub fn dbus_daemon(kind: &str, tmpdir: &Path) -> SupervisedChild {
    let config_path = tmpdir.join(format!("{}-dbus.xml", kind));
    let sock_path = tmpdir.join(format!("{}.sock", kind));
    let dbus_path = format!("unix:path={}", sock_path.display());
    std::fs::write(
        &config_path,
        format!(
            r#"
    <!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-Bus Bus Configuration 1.0//EN"
     "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
    <busconfig>
      <type>{}</type>
      <keep_umask/>
      <listen>{}</listen>
      <policy context="default">
        <allow send_destination="*" eavesdrop="true"/>
        <allow eavesdrop="true"/>
        <allow own="*"/>
      </policy>
    </busconfig>
    "#,
            kind, dbus_path
        ),
    )
    .expect("failed to write dbus config");

    std::env::set_var(
        format!("DBUS_{}_BUS_ADDRESS", kind.to_uppercase()),
        dbus_path,
    );
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

#[derive(Clone)]
pub struct AccountsFixtureOptions {
    pub num_users: Option<u32>,
    pub users: Vec<UserFixture>,
    pub cached_users: Option<Vec<String>>,
    pub homed_users: Vec<String>,
}

impl Default for AccountsFixtureOptions {
    fn default() -> Self {
        Self {
            num_users: None,
            users: vec![
                UserFixture::new("Phoshi", "phoshi", "phoshi.png", true, false),
                UserFixture::new("Guido", "agx", "guido.png", true, false),
                UserFixture::new("Sam", "samcday", "samcday.jpeg", true, false),
            ],
            cached_users: None,
            homed_users: vec![],
        }
    }
}

struct AccountsFixture {
    state: Arc<Mutex<FixtureState>>,
}

#[derive(Clone)]
pub struct UserFixture {
    name: String,
    username: String,
    icon_file: String,
    local_account: bool,
    system_account: bool,
}

#[derive(Default)]
struct FixtureState {
    users_by_name: BTreeMap<String, UserFixture>,
    cached_users: Vec<String>,
    homed_users: Vec<String>,
}

#[zbus::interface(name = "org.freedesktop.Accounts")]
impl AccountsFixture {
    async fn list_cached_users(&self) -> Vec<ObjectPath<'_>> {
        let state = self.state.lock().unwrap();
        state
            .cached_users
            .iter()
            .map(|name| ObjectPath::try_from(format!("/org/freedesktop/Accounts/{name}")).unwrap())
            .collect::<Vec<_>>()
    }

    async fn cache_user(&self, name: &str) -> zbus::fdo::Result<ObjectPath<'_>> {
        let mut state = self.state.lock().unwrap();
        if !state.cached_users.contains(&name.to_string()) && state.users_by_name.contains_key(name)
        {
            state.cached_users.push(name.to_string());
        }

        Ok(ObjectPath::try_from(format!("/org/freedesktop/Accounts/{name}")).unwrap())
    }
}

impl UserFixture {
    pub fn new(
        name: &str,
        username: &str,
        icon_file: &str,
        local_account: bool,
        system_account: bool,
    ) -> Self {
        Self {
            name: name.into(),
            username: username.into(),
            icon_file: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/")
                .join(icon_file)
                .display()
                .to_string(),
            local_account,
            system_account,
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
    #[zbus(property)]
    async fn local_account(&self) -> bool {
        self.local_account
    }
    #[zbus(property)]
    async fn system_account(&self) -> bool {
        self.system_account
    }
}

struct Home1ManagerFixture {
    state: Arc<Mutex<FixtureState>>,
}

#[zbus::interface(name = "org.freedesktop.home1.Manager")]
impl Home1ManagerFixture {
    async fn list_homes(
        &self,
    ) -> Vec<(
        String,
        u32,
        String,
        u32,
        String,
        String,
        String,
        ObjectPath<'_>,
    )> {
        let state = self.state.lock().unwrap();
        state
            .homed_users
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    1000,
                    "active".to_string(),
                    1000,
                    name.clone(),
                    format!("/home/{name}"),
                    "/bin/sh".to_string(),
                    ObjectPath::from_static_str_unchecked("/org/freedesktop/home1/home"),
                )
            })
            .collect()
    }
}

pub async fn run_accounts_fixture(
    connection: zbus::Connection,
    num_users: Option<u32>,
) -> anyhow::Result<()> {
    run_accounts_fixture_with_options(
        connection,
        AccountsFixtureOptions {
            num_users,
            ..Default::default()
        },
    )
    .await
}

pub async fn run_accounts_fixture_with_options(
    connection: zbus::Connection,
    options: AccountsFixtureOptions,
) -> anyhow::Result<()> {
    let mut cached_users = options.cached_users.unwrap_or_else(|| {
        options
            .users
            .iter()
            .map(|user| user.username.clone())
            .collect()
    });
    if let Some(num_users) = options.num_users {
        cached_users.truncate(num_users as usize);
    }

    let users_by_name = options
        .users
        .iter()
        .map(|user| (user.username.clone(), user.clone()))
        .collect();

    let state = Arc::new(Mutex::new(FixtureState {
        users_by_name,
        cached_users,
        homed_users: options.homed_users,
    }));

    connection
        .object_server()
        .at(
            "/org/freedesktop/Accounts",
            AccountsFixture {
                state: state.clone(),
            },
        )
        .await
        .context("failed to serve org.freedesktop.Accounts")?;

    for user in options.users {
        connection
            .object_server()
            .at(format!("/org/freedesktop/Accounts/{}", user.username), user)
            .await
            .context("failed to serve org.freedesktop.Accounts.User")?;
    }

    connection
        .object_server()
        .at(
            "/org/freedesktop/home1",
            Home1ManagerFixture {
                state: state.clone(),
            },
        )
        .await
        .context("failed to serve org.freedesktop.home1.Manager")?;

    connection
        .request_name("org.freedesktop.Accounts")
        .await
        .context("failed to request name")?;
    connection
        .request_name("org.freedesktop.home1")
        .await
        .context("failed to request home1 name")?;
    Ok(())
}
