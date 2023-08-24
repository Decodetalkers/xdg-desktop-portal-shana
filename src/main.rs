mod backends;
mod config;
mod protaltypes;
use backends::*;
use config::Config;
use protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};
use std::{collections::HashMap, error::Error, future::pending};
use zbus::{
    dbus_interface, fdo,
    zvariant::{ObjectPath, OwnedValue, Value},
    ConnectionBuilder,
};

use std::sync::OnceLock;

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

struct Shana {
    backendconfig: ProtalConfig,
}

#[derive(PartialEq, Debug, Eq)]
struct ProtalConfig {
    savefile: PortalSelect,
    openfile: PortalSelect,
    openfile_casefolder: PortalSelect,
}

impl PortalSelect {
    fn service_path(&self) -> &str {
        match self {
            PortalSelect::Kde => "org.freedesktop.impl.portal.desktop.kde",
            PortalSelect::Gnome => "org.freedesktop.impl.portal.desktop.gnome",
            PortalSelect::Lxqt => "org.freedesktop.impl.portal.desktop.lxqt",
            PortalSelect::Gtk => "org.freedesktop.impl.portal.desktop.gtk",
            PortalSelect::Other(path) => path,
        }
    }
}

#[dbus_interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl Shana {
    async fn open_file(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: OpenFileOptions,
    ) -> fdo::Result<(u32, SelectedFiles)> {
        let connection = get_connection().await?;
        let portal_select = if let Some(true) = options.directory {
            &self.backendconfig.openfile_casefolder
        } else {
            &self.backendconfig.openfile
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
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> fdo::Result<(u32, SelectedFiles)> {
        let connection = get_connection().await?;
        let portal = XdgDesktopFilePortalProxy::builder(&connection)
            .destination(self.backendconfig.savefile.service_path())?
            .build()
            .await?;

        let output = portal
            .save_file(handle, app_id, parent_window, title, options)
            .await?;
        Ok(output)
    }

    async fn save_files(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: HashMap<String, Value<'_>>,
    ) -> fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        let connection = get_connection().await?;
        // INFO: only gtk have savefiles, so if not use gnome or gtk, all fallback to gtk
        let portal = XdgDesktopFilePortalProxy::builder(&connection)
            .destination(PortalSelect::Gtk.service_path())?
            .build()
            .await?;
        let output = portal
            .save_files(handle, app_id, parent_window, title, options)
            .await?;
        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "xdg-desktop-protal-shana=info");
    tracing_subscriber::fmt().init();
    tracing::info!("Shana Start");
    let config = Config::config_from_file();
    let backendconfig = ProtalConfig::from(config);
    let _conn = ConnectionBuilder::session()?
        .name("org.freedesktop.impl.portal.desktop.shana")?
        .serve_at("/org/freedesktop/portal/desktop", Shana { backendconfig })?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
