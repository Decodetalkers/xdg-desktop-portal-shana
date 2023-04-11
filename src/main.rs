mod backends;
mod config;
mod protaltypes;
use backends::*;
use config::Config;
use protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};
use std::{collections::HashMap, error::Error, future::pending};
use tracing::{info, Level};
use zbus::{
    dbus_interface,
    zvariant::{ObjectPath, OwnedValue, Value},
    ConnectionBuilder,
};

struct Shana {
    backendconfig: ProtalConfig,
}

struct ProtalConfig {
    savefile: PortalSelect,
    openfile: PortalSelect,
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
        } else if self.backendconfig.openfile == PortalSelect::Lxqt {
            let Ok(proxy) = XdgDesktopLxqtProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.open_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else if self.backendconfig.openfile == PortalSelect::Kde {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.open_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopGtkProxy::new(&connection).await else {
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
        } else if self.backendconfig.savefile == PortalSelect::Lxqt {
            let Ok(proxy) = XdgDesktopLxqtProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.save_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else if self.backendconfig.savefile == PortalSelect::Kde {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, SelectedFiles::default());
            };
            let Ok(output) = proxy.save_file(handle, app_id, parent_window, title, options).await else {
                return (0, SelectedFiles::default());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopGtkProxy::new(&connection).await else {
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
    ) -> (u32, HashMap<String, OwnedValue>) {
        let Ok(connection) = zbus::Connection::session().await else {
            return (0, HashMap::new());
        };
        // INFO: only gtk and gnome have savefiles, so if not use gnome or gtk, all fallback to gtk
        if self.backendconfig.savefile == PortalSelect::Gnome {
            let Ok(proxy) = XdgDesktopKdeProxy::new(&connection).await else {
                return (0, HashMap::new());
            };
            let Ok(output) = proxy.save_files(handle, app_id, parent_window, title, options).await else {
                return (0, HashMap::new());
            };
            output
        } else {
            let Ok(proxy) = XdgDesktopGtkProxy::new(&connection).await else {
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
        .serve_at("/org/freedesktop/portal/desktop", Shana { backendconfig })?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
