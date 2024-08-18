use std::{
    ffi::{CString, OsStr},
    os::unix::ffi::OsStrExt,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

/// A file name represented as a nul-terminated byte array.
#[derive(Type, Debug, Default, PartialEq)]
#[zvariant(signature = "ay")]
pub struct FilePath(CString);

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        OsStr::from_bytes(self.0.as_bytes()).as_ref()
    }
}

impl Serialize for FilePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes_with_nul())
    }
}

impl<'de> Deserialize<'de> for FilePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        let c_string = CString::from_vec_with_nul(bytes)
            .map_err(|_| serde::de::Error::custom("Bytes are not nul-terminated"))?;

        Ok(Self(c_string))
    }
}

// SelectedFiles
#[derive(SerializeDict, DeserializeDict, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct SelectedFiles {
    pub uris: Vec<url::Url>,
    pub choices: Option<Vec<(String, String)>>,
    pub current_filter: Option<FileFilter>,
    // Only relavant for OpenFile
    pub writable: Option<bool>,
}

#[allow(dead_code)]
impl SelectedFiles {
    pub fn from_path(path: Vec<impl AsRef<Path>>) -> Self {
        Self {
            uris: path
                .iter()
                .map(url::Url::from_file_path)
                .filter_map(|urlunit| urlunit.ok())
                .collect(),
            choices: None,
            current_filter: None,
            writable: None,
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

#[allow(dead_code)]
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
    accept_label: Option<String>,
    modal: Option<bool>,
    multiple: Option<bool>,
    pub directory: Option<bool>,
    filters: Option<Vec<FileFilter>>,
    current_filter: Option<FileFilter>,
    choices: Option<Vec<Choice>>,
    current_folder: Option<FilePath>,
}

#[allow(dead_code)]
impl OpenFileOptions {
    pub fn select_function(&self) -> SelectFunction {
        if let Some(true) = self.directory {
            SelectFunction::Folder {
                title: self.accept_label.clone().unwrap_or("OpenFold".to_string()),
                filters: self.filters.as_ref().unwrap_or(&vec![]).clone(),
                current_filter: self.current_filter.clone(),
            }
        } else {
            SelectFunction::File {
                title: self.accept_label.clone().unwrap_or("OpenFile".to_string()),
                filters: (self.filters.as_ref().unwrap_or(&vec![])).clone(),
                current_filter: self.current_filter.clone(),
            }
        }
    }
}

#[allow(dead_code)]
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
    accept_label: Option<String>,
    modal: Option<bool>,
    multiple: Option<bool>,
    filters: Option<Vec<FileFilter>>,
    current_filter: Option<FileFilter>,
    choices: Option<Vec<Choice>>,
    current_name: Option<String>,
    current_folder: Option<FilePath>,
    current_file: Option<FilePath>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct SaveFilesOptions {
    accept_label: Option<String>,
    modal: Option<bool>,
    choices: Option<Vec<Choice>>,
    current_folder: Option<FilePath>,
    files: Option<Vec<FilePath>>,
}
