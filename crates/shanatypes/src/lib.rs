use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};
// SelectedFiles
#[derive(SerializeDict, Type, Debug, Default)]
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
                .map(|pathunit| url::Url::from_file_path(pathunit))
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

// filters contains all filters can only can select one at one time
#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    pub accept_label: Option<String>, // WindowTitle
    modal: Option<bool>,              // bool
    multiple: Option<bool>,           // bool
    pub directory: Option<bool>,
    pub filters: Vec<FileFilter>, // Filter
    pub current_filter: Option<FileFilter>,
    choices: Vec<Choice>,
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
    choices: Vec<Choice>,
    current_name: Option<String>,
    current_folder: Option<Vec<u8>>,
    current_file: Option<Vec<u8>>,
}
