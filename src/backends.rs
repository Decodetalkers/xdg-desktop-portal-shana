use serde::{Deserialize, Serialize};
use zbus::{
    proxy,
    zvariant::{self, ObjectPath},
};

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone)]
pub enum PortalSelect {
    Kde,
    Gnome,
    Lxqt,
    Gtk,
    Native,
    Other(String),
}
use crate::protaltypes::{OpenFileOptions, SaveFileOptions, SaveFilesOptions, SelectedFiles};

#[derive(zvariant::Type, Deserialize, Serialize, Clone, Copy)]
#[zvariant(signature = "(ua{sv})")]
pub enum PortalResponse<T: zvariant::Type + serde::Serialize> {
    Success(T),
    Cancelled,
    Other,
}

#[proxy(
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
    ) -> zbus::Result<PortalResponse<SelectedFiles>>;
    fn save_file(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> zbus::Result<PortalResponse<SelectedFiles>>;
    fn save_files(
        &self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFilesOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
}
