use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};
// SelectedFiles
#[derive(SerializeDict, DeserializeDict, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct SelectedFiles {
    pub uris: Vec<url::Url>,
    pub choices: Option<Vec<(String, String)>>,
}
impl SelectedFiles {
    pub fn from_path(path: Vec<impl AsRef<Path>>) -> Self {
        Self {
            uris: path
                .iter()
                .map(url::Url::from_file_path)
                .filter_map(|urlunit| urlunit.ok())
                .collect(),
            choices: None,
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Type, Debug)]
/// Presents the user with a choice to select from or as a checkbox.
pub struct Choice(String, String, Vec<(String, String)>, String);

#[derive(Clone, Serialize_repr, Deserialize, Debug, Type)]
#[repr(u32)]
pub enum FilterType {
    GlobPattern = 0,
    MimeType = 1,
}

#[derive(Clone, Serialize, Deserialize, Type, Debug)]
pub struct FileFilter(String, Vec<(FilterType, String)>);

impl FileFilter {
    pub fn get_filters(&self) -> Vec<(FilterType, String)> {
        self.1.clone()
    }
    pub fn get_name(&self) -> String {
        self.0.clone()
    }
}

// filters contains all filters can only can select one at one time
#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    accept_label: Option<String>, // WindowTitle
    modal: Option<bool>,          // bool
    multiple: Option<bool>,       // bool
    directory: Option<bool>,
    filters: Vec<FileFilter>, // Filter
    current_filter: Option<FileFilter>,
    choices: Option<Vec<Choice>>,
}

impl OpenFileOptions {
    pub fn select_function(&self) -> SelectFunction {
        if let Some(true) = self.directory {
            SelectFunction::Folder {
                title: self.accept_label.clone().unwrap_or("OpenFold".to_string()),
                filters: self.filters.clone(),
                current_filter: self.current_filter.clone(),
            }
        } else {
            SelectFunction::File {
                title: self.accept_label.clone().unwrap_or("OpenFile".to_string()),
                filters: self.filters.clone(),
                current_filter: self.current_filter.clone(),
            }
        }
    }
}

pub enum SelectFunction {
    File {
        title: String,
        filters: Vec<FileFilter>,
        current_filter: Option<FileFilter>,
    },
    Folder {
        title: String,
        filters: Vec<FileFilter>,
        current_filter: Option<FileFilter>,
    },
}

// filters contains all filters can only can select one at one time
#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct SaveFileOptions {
    accept_label: Option<String>, // String
    modal: Option<bool>,          // bool
    multiple: Option<bool>,       // bool
    filters: Vec<FileFilter>,
    current_filter: Option<FileFilter>,
    choices: Option<Vec<Choice>>,
    current_name: Option<String>,
    current_folder: Option<Vec<u8>>,
    current_file: Option<Vec<u8>>,
}
