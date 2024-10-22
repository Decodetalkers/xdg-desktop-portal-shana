use zbus::{proxy, zvariant::ObjectPath};

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone)]
pub enum PortalSelect {
    Kde,
    Gnome,
    Lxqt,
    Gtk,
    Other(String),
}
use crate::protaltypes::{OpenFileOptions, SaveFileOptions, SaveFilesOptions, SelectedFiles};

#[proxy(
    interface = "org.freedesktop.impl.portal.FileChooser",
    default_path = "/org/freedesktop/portal/desktop"
)]
pub trait XdgDesktopFilePortal {
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
        options: SaveFilesOptions,
    ) -> zbus::Result<(u32, SelectedFiles)>;
}
