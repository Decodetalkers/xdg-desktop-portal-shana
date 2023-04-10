mod config;
mod protaltypes;
use protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};
use std::{error::Error, future::pending, collections::HashMap};
use tracing::{info, Level};
use zbus::{dbus_interface, dbus_proxy, zvariant::{ObjectPath, Value, OwnedValue}, ConnectionBuilder};
use config::Config;
struct Shana {
    backendconfig: ProtalConfig,
}

#[allow(unused)]
#[derive(PartialEq, PartialOrd)]
enum PortalSelect {
    Kde,
    Gnome,
}

struct ProtalConfig {
    savefile: PortalSelect,
    openfile: PortalSelect,
}

#[dbus_proxy(
    interface = "org.freedesktop.impl.portal.FileChooser",
    default_service = "org.freedesktop.impl.portal.desktop.kde",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgDesktopKde {
    fn open_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: OpenFileOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
    fn save_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
    fn save_files(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: HashMap<String, Value<'_>>,
    ) -> zbus::Result<(u32, HashMap<String, OwnedValue>)>;
}

#[dbus_proxy(
    interface = "org.freedesktop.impl.portal.FileChooser",
    default_service = "org.freedesktop.impl.portal.desktop.gnome",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgDesktopGnome {
    fn open_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: OpenFileOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
    fn save_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
    fn save_files(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: HashMap<String, Value<'_>>,
    ) -> zbus::Result<(u32, HashMap<String, OwnedValue>)>;
}

//mod filechoosertypes;
#[dbus_interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl Shana {
    async fn open_file(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: OpenFileOptions,
    ) -> (u32, SelectedFiles) {
        let Ok(connection) = zbus::Connection::session().await else {
            return (0, SelectedFiles::default());
        };
        if self.backendconfig.openfile == PortalSelect::Gnome {
            let Ok(proxy) = XdgDesktopGnomeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.open_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.open_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        }
    }

    async fn save_file(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> (u32, SelectedFiles) {
        let Ok(connection) = zbus::Connection::session().await else {
            return (0, SelectedFiles::default());
        };
        if self.backendconfig.savefile == PortalSelect::Gnome {
            let Ok(proxy) = XdgDesktopGnomeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.save_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.save_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        }
    }
    async fn save_files(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: HashMap<String, Value<'_>>,
    ) -> (u32, HashMap<String,OwnedValue>) {
        let Ok(connection) = zbus::Connection::session().await else {
            return (0, HashMap::new());
        };
        if self.backendconfig.savefile == PortalSelect::Gnome {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, HashMap::new());
            };
            let Ok(output) = proxy.save_files(handle, app_id, parent_window, title, options).await else {
                return (0, HashMap::new());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, HashMap::new());
            };
            let Ok(output) = proxy.save_files(handle, app_id, parent_window, title, options).await else {
                return (0, HashMap::new());
            };
            output
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "xdg-desktop-protal-shana=info");
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    info!("Shana Start");
    let config = Config::config_from_file();
    let backendconfig = ProtalConfig::from(config);
    let _conn = ConnectionBuilder::session()?
        .name("org.freedesktop.impl.portal.desktop.shana")?
        .serve_at(
            "/org/freedesktop/portal/desktop",
            Shana {
                backendconfig,
            },
        )?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
