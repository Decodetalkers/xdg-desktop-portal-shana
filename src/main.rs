use filedialogshana::choose_file;
use shanatypes::{OpenFileOptions, SaveFileOptions, SelectedFiles};
use std::{error::Error, future::pending};
use tracing::{info, Level};
use zbus::{dbus_interface, zvariant::ObjectPath, ConnectionBuilder};
struct Shana;
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
        println!("handle is {:?}", handle);
        println!("app_id is {}", app_id);
        println!("parent_window is {}", parent_window);
        println!("title is {}", title);
        println!("options is {:?}", options);
        let selected = options.select_function();
        choose_file(selected)
    }

    async fn save_file(
        &mut self,
        handle: ObjectPath<'_>,
        app_id: String,
        parent_window: String,
        title: String,
        options: SaveFileOptions,
    ) -> (u32, SelectedFiles) {
        println!("handle is {:?}", handle);
        println!("app_id is {}", app_id);
        println!("parent_window is {}", parent_window);
        println!("title is {}", title);
        println!("options is {:?}", options);
        (
            0,
            SelectedFiles {
                uris: vec![],
                choices: None,
            },
        )
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "xdg-desktop-protal-shana=info");
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    info!("Shana Start");
    ConnectionBuilder::session()?
        .name("org.freedesktop.impl.portal.desktop.shana")?
        .serve_at("/org/freedesktop/portal/desktop", Shana)?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}
