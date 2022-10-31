use glob::Pattern;
use once_cell::sync::Lazy;
use shanatypes::{FileFilter, FilterType};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::patterns::{ICONPATTERN, TEXTPATTERN};

pub static HOME: Lazy<String> = Lazy::new(|| std::env::var("HOME").unwrap());
pub static HIDDEN_FILE: Lazy<Pattern> = Lazy::new(|| Pattern::new(".*").unwrap());
mod permission_mods {
    pub type Mode = u32;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
}

pub enum ShowHow {
    OnlyVisible,
    ShowHidden,
    OnlyHidden,
    OnlyFolder,
    OnlyFile,
}

pub enum FileType {
    Folder {
        name: String,
        owner: String,
        viewable: bool,
        filepath: String,
        hidden: bool,
    },
    File {
        name: String,
        owner: String,
        readable: bool,
        runable: bool,
        filepath: String,
        mimetype: String,
        hidden: bool,
    },
}

impl FileType {
    fn is_mimetype(&self, input: &str) -> bool {
        match self {
            FileType::Folder { .. } => true,
            FileType::File { mimetype, .. } => mimetype == input,
        }
    }
    fn is_glob(&self, input: &str) -> bool {
        match self {
            Self::Folder { .. } => true,
            Self::File { name, .. } => {
                let pattern = Pattern::new(input).unwrap();
                pattern.matches(&name)
            }
        }
    }
    fn is_hidden(&self) -> bool {
        match self {
            Self::File { hidden, .. } => *hidden,
            Self::Folder { hidden, .. } => *hidden,
        }
    }
    fn is_folder(&self) -> bool {
        match self {
            Self::File { .. } => false,
            Self::Folder { .. } => true,
        }
    }

    pub fn source_type(&self) -> i32 {
        match self {
            FileType::Folder { .. } => 0,
            FileType::File { mimetype, .. } => {
                if ICONPATTERN.contains(&mimetype.as_str()) {
                    1
                } else if TEXTPATTERN.contains(&mimetype.as_str()) {
                    2
                } else {
                    0
                }
            }
        }
    }
}
//struct FileMessage {
//    filetype: FileType,
//    mimetype: Option<String>,
//}

fn folder_enterable(dir: impl AsRef<Path>) -> bool {
    match dir.as_ref().metadata() {
        Ok(metadata) => {
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

fn file_runable(dir: impl AsRef<Path>) -> bool {
    match dir.as_ref().metadata() {
        Ok(metadata) => {
            let bits = metadata.mode();
            let has_bit = |bit| bits & bit == bit;
            let current_user = users::get_current_uid();
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

pub fn get_files_from_folder(
    dir: impl AsRef<Path>,
    howshow: ShowHow,
    filiters: Option<Vec<FileFilter>>,
) -> Vec<FileType> {
    if dir.as_ref().is_dir() && folder_enterable(&dir) {
        let entry = {
            match std::fs::read_dir(dir) {
                Ok(dir) => dir.flatten(),
                Err(_) => return Vec::new(),
            }
        };
        entry
            .into_iter()
            .map(|fileunit| {
                let path = fileunit.path();
                let filepath = path.to_str().unwrap().to_string();
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let metedata = fileunit.metadata().unwrap();
                let owner = metedata.uid();
                let owner = users::get_user_by_uid(owner)
                    .unwrap()
                    .name()
                    .to_str()
                    .unwrap()
                    .to_string();
                if metedata.is_dir() {
                    FileType::Folder {
                        hidden: HIDDEN_FILE.matches(&name),
                        name,
                        owner,
                        viewable: folder_enterable(path),
                        filepath,
                    }
                } else {
                    let mimetype = gio::functions::content_type_guess(Some(&path), &[])
                        .0
                        .to_string();
                    FileType::File {
                        hidden: HIDDEN_FILE.matches(&name),
                        name,
                        owner,
                        readable: file_readable(&path),
                        runable: file_runable(&path),
                        filepath,
                        mimetype,
                    }
                }
            })
            .filter(|output| match howshow {
                ShowHow::ShowHidden => true,
                ShowHow::OnlyHidden => output.is_hidden(),
                ShowHow::OnlyVisible => !output.is_hidden(),
                ShowHow::OnlyFile => !output.is_folder(),
                ShowHow::OnlyFolder => output.is_folder(),
            })
            .filter(|output| match &filiters {
                None => true,
                Some(filters) => {
                    for filter in filters {
                        for fil in filter.get_filters() {
                            if !match fil.0 {
                                FilterType::MimeType => output.is_mimetype(&fil.1),
                                FilterType::GlobPattern => output.is_glob(&fil.1),
                            } {
                                return false;
                            }
                        }
                    }
                    true
                }
            })
            .collect()
    } else {
        vec![]
    }
}
