use std::collections::HashMap;
use zbus::{
    dbus_proxy,
    zvariant::{ObjectPath, OwnedValue, Value},
};

#[derive(PartialEq, PartialOrd)]
pub enum PortalSelect {
    Kde,
    Gnome,
    Lxqt,
    Gtk,
}
use crate::protaltypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};

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

#[dbus_proxy(
    interface = "org.freedesktop.impl.portal.FileChooser",
    default_service = "org.freedesktop.impl.portal.desktop.gtk",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgDesktopGtk {
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
    default_service = "org.freedesktop.impl.portal.desktop.lxqt",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgDesktopLxqt {
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
