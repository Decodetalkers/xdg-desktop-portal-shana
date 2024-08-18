use std::{
    ffi::{CString, OsStr},
    fmt::Display,
    os::unix::ffi::OsStrExt,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

/// A file name represented as a nul-terminated byte array.
#[derive(Type, Debug, Default, PartialEq, Clone)]
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
#[derive(SerializeDict, DeserializeDict, Type, Debug, Default, Clone)]
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
#[derive(Clone, Serialize_repr, Deserialize, Debug, Type, PartialEq, Eq)]
#[repr(u32)]
pub enum FilterType {
    GlobPattern = 0,
    MimeType = 1,
}

impl FilterType {
    /// Whether it is a mime type filter.
    fn is_mimetype(&self) -> bool {
        matches!(self, FilterType::MimeType)
    }

    /// Whether it is a glob pattern type filter.
    fn is_pattern(&self) -> bool {
        matches!(self, FilterType::GlobPattern)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct FileFilter(String, Vec<(FilterType, String)>);

impl FileFilter {
    pub fn title(&self) -> &str {
        &self.0
    }

    pub fn get_filters(&self) -> &[(FilterType, String)] {
        &self.1
    }
}

impl FileFilter {
    /// Create a new file filter
    ///
    /// # Arguments
    ///
    /// * `label` - user-visible name of the file filter.
    pub fn new(label: &str) -> Self {
        Self(label.to_owned(), vec![])
    }

    /// Adds a mime type to the file filter.
    #[must_use]
    pub fn mimetype(mut self, mimetype: &str) -> Self {
        self.1.push((FilterType::MimeType, mimetype.to_owned()));
        self
    }

    /// Adds a glob pattern to the file filter.
    #[must_use]
    pub fn glob(mut self, pattern: &str) -> Self {
        self.1.push((FilterType::GlobPattern, pattern.to_owned()));
        self
    }

    #[allow(unused)]
    pub(crate) fn filters(&self) -> &Vec<(FilterType, String)> {
        &self.1
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self("All files: (*)".to_string(), Vec::new())
    }
}

impl Display for FileFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_info = format!("{} :", self.0.clone());

        for (_, show) in self.1.iter() {
            display_info.push_str(&format!(" {}", show));
        }
        write!(f, "{}", display_info)
    }
}

impl FileFilter {
    /// The label of the filter.
    pub fn label(&self) -> &str {
        &self.0
    }

    /// List of mimetypes filters.
    pub fn mimetype_filters(&self) -> Vec<&str> {
        self.1
            .iter()
            .filter_map(|(type_, string)| type_.is_mimetype().then_some(string.as_str()))
            .collect()
    }

    /// List of glob patterns filters.
    pub fn pattern_filters(&self) -> Vec<&str> {
        self.1
            .iter()
            .filter_map(|(type_, string)| type_.is_pattern().then_some(string.as_str()))
            .collect()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
/// Presents the user with a choice to select from or as a checkbox.
pub struct Choice(String, String, Vec<(String, String)>, String);

impl Choice {
    /// Creates a checkbox choice.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier of the choice.
    /// * `label` - user-visible name of the choice.
    /// * `state` - the initial state value.
    pub fn boolean(id: &str, label: &str, state: bool) -> Self {
        Self::new(id, label, &state.to_string())
    }

    /// Creates a new choice.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier of the choice.
    /// * `label` - user-visible name of the choice.
    /// * `initial_selection` - the initially selected value.
    pub fn new(id: &str, label: &str, initial_selection: &str) -> Self {
        Self(
            id.to_owned(),
            label.to_owned(),
            vec![],
            initial_selection.to_owned(),
        )
    }

    /// Adds a (key, value) as a choice.
    #[must_use]
    pub fn insert(mut self, key: &str, value: &str) -> Self {
        self.2.push((key.to_owned(), value.to_owned()));
        self
    }

    /// The choice's unique id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// The user visible label of the choice.
    pub fn label(&self) -> &str {
        &self.1
    }

    /// Pairs of choices.
    pub fn pairs(&self) -> Vec<(&str, &str)> {
        self.2
            .iter()
            .map(|(x, y)| (x.as_str(), y.as_str()))
            .collect::<Vec<_>>()
    }

    /// The initially selected value.
    pub fn initial_selection(&self) -> &str {
        &self.3
    }
}

// filters contains all filters can only can select one at one time
#[derive(SerializeDict, DeserializeDict, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    pub accept_label: Option<String>, // WindowTitle
    pub modal: Option<bool>,          // bool
    pub multiple: Option<bool>,       // bool
    pub directory: Option<bool>,
    pub filters: Option<Vec<FileFilter>>, // Filter
    pub current_filter: Option<FileFilter>,
    pub choices: Option<Vec<Choice>>,
    pub current_folder: Option<FilePath>,
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
    pub accept_label: Option<String>, // String
    pub modal: Option<bool>,          // bool
    pub multiple: Option<bool>,       // bool
    pub filters: Vec<FileFilter>,
    pub current_filter: Option<FileFilter>,
    pub choices: Option<Vec<Choice>>,
    pub current_name: Option<String>,
    pub current_folder: Option<FilePath>,
    pub current_file: Option<FilePath>,
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
