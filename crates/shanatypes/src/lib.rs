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
#[derive(Clone, Serialize, Deserialize, Type, Debug)]
/// Presents the user with a choice to select from or as a checkbox.
pub struct Choice(String, String, Vec<(String, String)>, String);

#[derive(Clone, Serialize_repr, Deserialize, Debug, Type)]
#[repr(u32)]
enum FilterType {
    GlobPattern = 0,
    MimeType = 1,
}

#[derive(Clone, Serialize, Deserialize, Type, Debug)]
pub struct FileFilter(String, Vec<(FilterType, String)>);

#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    accept_label: Option<String>, // WindowTitle
    modal: Option<bool>,          // bool
    multiple: Option<bool>,       // bool
    directory: Option<bool>,
    filters: Vec<FileFilter>, // Filter
    current_filter: Option<FilterType>,
    choices: Vec<Choice>,
}
#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct SaveFileOptions {
    accept_label: Option<String>, // String
    modal: Option<bool>,          // bool
    multiple: Option<bool>,       // bool
    filters: Vec<FileFilter>,
    current_filter: Option<FilterType>,
    choices: Vec<Choice>,
    current_name: Option<String>,
    current_folder: Option<Vec<u8>>,
    current_file: Option<Vec<u8>>
}
