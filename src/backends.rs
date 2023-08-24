use std::collections::HashMap;
use zbus::{
    dbus_proxy,
    zvariant::{ObjectPath, OwnedValue, Value},
};

#[derive(PartialEq, PartialOrd, Eq, Debug)]
pub enum PortalSelect {
    Kde,
    Gnome,
    Lxqt,
    Gtk,
    Other(String),
}
use crate::protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};

#[dbus_proxy(
    interface = "org.freedesktop.impl.portal.FileChooser",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgDesktopFilePortal {
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
