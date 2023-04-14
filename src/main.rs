mod backends;
mod config;
mod protaltypes;
use backends::*;
use config::Config;
use once_cell::sync::OnceCell;
use protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};
use std::{collections::HashMap, error::Error, future::pending};
use zbus::{
    dbus_interface, fdo,
    zvariant::{ObjectPath, OwnedValue, Value},
    ConnectionBuilder,
};

static SESSION: OnceCell<zbus::Connection> = OnceCell::new();

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

struct ProtalConfig {
    savefile: PortalSelect,
    openfile: PortalSelect,
    openfile_casefolder: PortalSelect,
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
        if *portal_select == PortalSelect::Gnome {
            let proxy = XdgDesktopGnomeProxy::new(&connection).await?;
            let output = proxy
                .open_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else if *portal_select == PortalSelect::Lxqt {
            let proxy = XdgDesktopLxqtProxy::new(&connection).await?;
            let output = proxy
                .open_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else if *portal_select == PortalSelect::Kde {
            let proxy = XdgDesktopKdeProxy::new(&connection).await?;
            let output = proxy
                .open_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else {
            let proxy = XdgDesktopGtkProxy::new(&connection).await?;
            let output = proxy
                .open_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        }
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
        if self.backendconfig.savefile == PortalSelect::Gnome {
            let proxy = XdgDesktopGnomeProxy::new(&connection).await?;
            let output = proxy
                .save_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else if self.backendconfig.savefile == PortalSelect::Lxqt {
            let proxy = XdgDesktopLxqtProxy::new(&connection).await?;
            let output = proxy
                .save_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else if self.backendconfig.savefile == PortalSelect::Kde {
            let proxy = XdgDesktopKdeProxy::new(&connection).await?;
            let output = proxy
                .save_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        } else {
            let proxy = XdgDesktopGtkProxy::new(&connection).await?;
            let output = proxy
                .save_file(handle, app_id, parent_window, title, options)
                .await?;
            Ok(output)
        }
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
        let proxy = XdgDesktopGtkProxy::new(&connection).await?;
        let output = proxy
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
