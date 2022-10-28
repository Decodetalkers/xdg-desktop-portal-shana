use std::path::Path;
slint::include_modules!();
mod permission_mods {
    pub type Mode = u32;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
}

enum FileType {
    Fold {
        owner: String,
        viewable: bool,
    },
    File {
        owner: String,
        permission: permission_mods::Mode,
    },
}

struct FileMessage {
    filetype: FileType,
    mimetype: Option<String>,
}

fn folder_enterable(dir: impl AsRef<Path>) -> bool {
    match dir.as_ref().metadata() {
        Ok(metadata) => {
            use std::os::unix::fs::MetadataExt;
            let bits = metadata.mode();
            let has_bit = |bit| bits & bit == bit;
            let current_user = users::get_current_uid();
            if current_user == 0 {
                return true;
            }
            let current_group = users::get_current_gid();
            let owner_user = metadata.uid();
            let owner_group = metadata.gid();
            match (current_user == owner_user, current_group == owner_group) {
                (true, _) => has_bit(permission_mods::USER_EXECUTE),
                (false, true) => has_bit(permission_mods::GROUP_EXECUTE),
                (false, false) => has_bit(permission_mods::OTHER_EXECUTE),
            }
        }
        Err(_) => false,
    }
}

fn file_readable(dir: impl AsRef<Path>) -> bool {
    match dir.as_ref().metadata() {
        Ok(metadata) => {
            use std::os::unix::fs::MetadataExt;
            let bits = metadata.mode();
            let has_bit = |bit| bits & bit == bit;
            let current_user = users::get_current_uid();
            if current_user == 0 {
                return true;
            }
            let current_group = users::get_current_gid();
            let owner_user = metadata.uid();
            let owner_group = metadata.gid();
            match (current_user == owner_user, current_group == owner_group) {
                (true, _) => has_bit(permission_mods::USER_READ),
                (false, true) => has_bit(permission_mods::GROUP_READ),
                (false, false) => has_bit(permission_mods::OTHER_READ),
            }
        }
        Err(_) => false,
    }
}
use std::sync::mpsc;

use shanatypes::SelectedFiles;
// 0 is succeed , 1 is cancel , 2 is other
pub fn choose_file() -> (u32, SelectedFiles) {
    let (stdout_tx, stdout_rs) = mpsc::channel();
    let ui = AppWindow::new();

    ui.on_request_increase_value(move || {
        let _ = stdout_tx.send((
            2,
            SelectedFiles {
                uris: vec![],
                choices: None,
            },
        ));
        let _ = slint::quit_event_loop();
    });

    ui.run();
    if let Ok(output) = stdout_rs.recv_timeout(std::time::Duration::from_nanos(300)) {
        output
    } else {
        (
            2,
            SelectedFiles {
                uris: vec![],
                choices: None,
            },
        )
    }
}
