mod backends;
mod config;
mod protaltypes;
use backends::*;
use config::Config;
use notify::EventKind;
use protaltypes::{OpenFileOptions, SaveFileOptions, SaveFilesOptions, SelectedFiles};
use std::{error::Error, future::pending, sync::Arc};
use tokio::sync::Mutex;
use zbus::{connection, fdo, interface, zvariant::ObjectPath};

use std::sync::OnceLock;

use std::sync::LazyLock;

use futures::{
    SinkExt, StreamExt,
    channel::mpsc::{Receiver, channel},
};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

static SETTING_CONFIG: LazyLock<Arc<Mutex<ProtalConfig>>> =
    LazyLock::new(|| Arc::new(Mutex::new(ProtalConfig::from(Config::config_from_file()))));

async fn get_setting_config() -> ProtalConfig {
    let config = SETTING_CONFIG.lock().await;
    config.clone()
}

async fn update_setting_config() {
    let mut config = SETTING_CONFIG.lock().await;
    *config = ProtalConfig::from(Config::config_from_file());
}

static SESSION: OnceLock<zbus::Connection> = OnceLock::new();

async fn get_connection() -> zbus::Result<zbus::Connection> {
    if let Some(cnx) = SESSION.get() {
        Ok(cnx.clone())
    } else {
        let cnx = zbus::Connection::session().await?;
        SESSION.set(cnx.clone()).expect("Can't reset a OnceCell");
        Ok(cnx)
    }
}

struct Shana;

#[derive(PartialEq, Debug, Eq, Clone)]
struct ProtalConfig {
    savefile: PortalSelect,
    openfile: PortalSelect,
    savefiles: PortalSelect,
    openfile_casefolder: PortalSelect,
}

impl PortalSelect {
    fn service_path(&self) -> &str {
        match self {
            PortalSelect::Kde => "org.freedesktop.impl.portal.desktop.kde",
            PortalSelect::Gnome => "org.gnome.Nautilus",
            PortalSelect::Lxqt => "org.freedesktop.impl.portal.desktop.lxqt",
            PortalSelect::Gtk => "org.freedesktop.impl.portal.desktop.gtk",
            PortalSelect::Other(path) => path,
        }
    }
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl Shana {
    async fn open_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: OpenFileOptions,
    ) -> fdo::Result<(u32, SelectedFiles)> {
        let connection = get_connection().await?;
        let backendconfig = get_setting_config().await;
        let portal_select = if let Some(true) = options.directory {
            backendconfig.openfile_casefolder
        } else {
            backendconfig.openfile
        };
        let portal = XdgDesktopFilePortalProxy::builder(&connection)
            .destination(portal_select.service_path())?
            .build()
            .await?;

        let output = portal
            .open_file(handle, app_id, parent_window, title, options)
            .await?;

        Ok(output)
    }

    async fn save_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> fdo::Result<(u32, SelectedFiles)> {
        let connection = get_connection().await?;
        let backendconfig = get_setting_config().await;
        let portal = XdgDesktopFilePortalProxy::builder(&connection)
            .destination(backendconfig.savefile.service_path())?
            .build()
            .await?;

        let output = portal
            .save_file(handle, app_id, parent_window, title, options)
            .await?;
        Ok(output)
    }

    async fn save_files(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFilesOptions,
    ) -> fdo::Result<(u32, SelectedFiles)> {
        let connection = get_connection().await?;
        let backendconfig = get_setting_config().await;
        // INFO: only gtk have savefiles, so if not use gnome or gtk, all fallback to gtk
        let portal = XdgDesktopFilePortalProxy::builder(&connection)
            .destination(backendconfig.savefiles.service_path())?
            .build()
            .await?;
        let output = portal
            .save_files(handle, app_id, parent_window, title, options)
            .await?;
        Ok(output)
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        notify::Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(Event {
                kind: EventKind::Modify(_),
                ..
            })
            | Ok(Event {
                kind: EventKind::Create(_),
                ..
            }) => {
                update_setting_config().await;
            }
            Err(e) => println!("watch error: {:?}", e),
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    unsafe { std::env::set_var("RUST_LOG", "xdg-desktop-protal-shana=info") };
    tracing_subscriber::fmt().init();
    tracing::info!("Shana Start");
    let _conn = connection::Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.shana")?
        .serve_at("/org/freedesktop/portal/desktop", Shana)?
        .build()
        .await?;
    tokio::spawn(async move {
        let Ok(home) = std::env::var("HOME") else {
            return;
        };
        let config_path = std::path::Path::new(home.as_str())
            .join(".config")
            .join("xdg-desktop-portal-shana");
        if let Err(e) = async_watch(config_path).await {
            tracing::info!("Maybe config file is not exist, create one :{e}");
        }
    });

    pending::<()>().await;
    Ok(())
}
