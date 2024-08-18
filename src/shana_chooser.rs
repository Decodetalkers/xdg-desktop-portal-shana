mod dirfs;
mod icon_cache;
mod utils;

use crate::protaltypes::{
    Choice, FileFilter, FilePath, OpenFileOptions, SaveFileOptions, SelectedFiles,
};
use crate::{PortalConfig, PortalResponse};

use dirfs::{update_dir_infos, DirUnit, FsInfo};
use iced::widget::{checkbox, column, combo_box, container, row, scrollable, text, Column, Space};
use iced::window::Id;
use iced::{executor, Length};
use iced::{Command, Element, Theme};
use std::path::{Path, PathBuf};

use iced_layershell::reexport::Anchor;
use iced_layershell::Application;
use iced_runtime::command::Action;
use iced_runtime::window::Action as WindowAction;

use iced_layershell::settings::{LayerShellSettings, Settings};

use iced_aw::{split, Split};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum FileChosen {
    OpenFile {
        accept_label: String,
        modal: bool,
        multiple: bool,
        directory: bool,
        filters: Vec<FileFilter>,
        current_filter: Option<FileFilter>,
        choices: Vec<Choice>,
        current_folder: Option<FilePath>,
    },
    SaveFile {
        current_name: String,
        accept_label: String,
        modal: bool,
        filters: Vec<FileFilter>,
        current_filter: Option<FileFilter>,
        choices: Vec<Choice>,
        current_folder: Option<FilePath>,
        current_file: Option<FilePath>,
    },
}

impl Default for FileChosen {
    fn default() -> Self {
        Self::OpenFile {
            accept_label: "".to_string(),
            modal: true,
            multiple: false,
            directory: false,
            filters: Vec::new(),
            current_filter: None,
            choices: Vec::new(),
            current_folder: None,
        }
    }
}

impl FileChosen {
    pub fn is_filechooser(&self) -> bool {
        matches!(self, FileChosen::OpenFile { .. })
    }

    pub fn is_multi_filechooser(&self) -> bool {
        matches!(self, FileChosen::OpenFile { multiple: true, .. })
    }

    pub fn is_savefile(&self) -> bool {
        !self.is_filechooser()
    }

    pub fn is_directory(&self) -> bool {
        matches!(
            self,
            FileChosen::OpenFile {
                directory: true,
                ..
            }
        )
    }

    pub fn filters(&self) -> &[FileFilter] {
        match self {
            Self::OpenFile { filters, .. } => filters,
            Self::SaveFile { filters, .. } => filters,
        }
    }

    pub fn choices(&self) -> &[Choice] {
        match self {
            Self::OpenFile { choices, .. } => choices,
            Self::SaveFile { choices, .. } => choices,
        }
    }

    pub fn accept_label(&self) -> &str {
        match self {
            Self::OpenFile { accept_label, .. } => accept_label,
            Self::SaveFile { accept_label, .. } => accept_label,
        }
    }

    pub fn current_filter(&self) -> Option<&FileFilter> {
        match self {
            Self::OpenFile { current_filter, .. } => current_filter.as_ref(),
            Self::SaveFile { current_filter, .. } => current_filter.as_ref(),
        }
    }

    pub fn is_modal(&self) -> bool {
        match self {
            Self::OpenFile { modal, .. } => *modal,
            Self::SaveFile { modal, .. } => *modal,
        }
    }
}

pub struct ShanaFileChooser {
    dir: DirUnit,
    display_name: String,
    showhide: bool,
    preview_big_image: bool,
    selected_paths: Vec<PathBuf>,
    current_selected: Option<PathBuf>,
    right_splitter: Option<u16>,
    left_splitter: Option<u16>,
    choose_option: FileChosen,
    current_filter: FileFilter,
    filters: combo_box::State<FileFilter>,
    response: std::sync::Arc<std::sync::Mutex<PortalResponse<SelectedFiles>>>,
}

pub fn open_file_native(
    OpenFileOptions {
        accept_label,
        modal,
        multiple,
        directory,
        filters,
        current_filter,
        choices,
        current_folder,
    }: OpenFileOptions,
) -> PortalResponse<SelectedFiles> {
    use std::sync::Arc;
    use std::sync::Mutex;
    let response: Arc<Mutex<PortalResponse<SelectedFiles>>> =
        Arc::new(Mutex::new(PortalResponse::Cancelled));
    let Ok(_) = ShanaFileChooser::run(Settings {
        layer_settings: LayerShellSettings {
            margin: (200, 200, 200, 200),
            anchor: Anchor::Left | Anchor::Right | Anchor::Top | Anchor::Bottom,
            ..Default::default()
        },
        flags: InitParm {
            response: response.clone(),
            choose_option: FileChosen::OpenFile {
                accept_label: accept_label.unwrap_or_default(),
                modal: modal.unwrap_or(false),
                multiple: multiple.unwrap_or(false),
                directory: directory.unwrap_or(false),
                filters: filters.unwrap_or_default(),
                current_filter,
                choices: choices.unwrap_or_default(),
                current_folder,
            },
        },
        ..Default::default()
    }) else {
        return PortalResponse::Other;
    };

    let res = response.lock().unwrap().clone();
    drop(response);
    res
}

#[allow(unused)]
pub fn save_file_native(
    SaveFileOptions {
        accept_label,
        modal,
        multiple: _,
        filters,
        current_filter,
        choices,
        current_name,
        current_folder,
        current_file,
    }: SaveFileOptions,
) -> PortalResponse<SelectedFiles> {
    use std::sync::Arc;
    use std::sync::Mutex;
    let response: Arc<Mutex<PortalResponse<SelectedFiles>>> =
        Arc::new(Mutex::new(PortalResponse::Cancelled));
    let Ok(_) = ShanaFileChooser::run(Settings {
        layer_settings: LayerShellSettings {
            margin: (200, 200, 200, 200),
            anchor: Anchor::Left | Anchor::Right | Anchor::Top | Anchor::Bottom,
            ..Default::default()
        },
        flags: InitParm {
            response: response.clone(),
            choose_option: FileChosen::SaveFile {
                accept_label: accept_label.unwrap_or_default(),
                modal: modal.unwrap_or(false),
                filters,
                current_filter,
                choices: choices.unwrap_or_default(),
                current_folder,
                current_file,
                current_name: current_name.unwrap_or_default(),
            },
        },
        ..Default::default()
    }) else {
        return PortalResponse::Other;
    };

    let res = response.lock().unwrap().clone();
    drop(response);
    res
}

fn is_samedir(patha: &Path, pathb: &Path) -> bool {
    let Ok(origin_path) = patha.canonicalize() else {
        return false;
    };
    let Ok(self_path) = pathb.canonicalize() else {
        return false;
    };
    self_path.as_os_str() == origin_path.as_os_str()
}

#[derive(Debug, Clone)]
pub enum Message {
    RequestMultiSelect((bool, PathBuf)),
    RequestNextDirs((Vec<FsInfo>, PathBuf)),
    RequestSelect(PathBuf),
    RequestEnter(PathBuf),
    RequestShowHide(bool),
    RequestShowImage(bool),
    RequestAdjustRightSplitter(u16),
    RequestAdjustLeftSplitter(u16),
    SearchPatternCachedChanged(String),
    SearchPatternChanged,

    FilterChanged(FileFilter),
    // CONFIRM
    Confirm,
    Cancel,
}

pub struct InitParm {
    response: std::sync::Arc<std::sync::Mutex<PortalResponse<SelectedFiles>>>,
    choose_option: FileChosen,
}

impl Default for InitParm {
    fn default() -> Self {
        Self {
            response: std::sync::Arc::new(std::sync::Mutex::new(PortalResponse::Cancelled)),
            choose_option: FileChosen::default(),
        }
    }
}

impl Application for ShanaFileChooser {
    type Message = Message;
    type Flags = InitParm;
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(
        InitParm {
            response,
            choose_option,
        }: Self::Flags,
    ) -> (Self, Command<Message>) {
        let mut filters = [FileFilter::default()].to_vec();
        let mut input_filters = choose_option.filters().to_vec();
        filters.append(&mut input_filters);
        (
            Self {
                dir: DirUnit::enter(std::env::current_dir().unwrap().as_path()),
                display_name: choose_option.accept_label().to_string(),
                showhide: false,
                preview_big_image: false,
                selected_paths: Vec::new(),
                current_selected: None,
                right_splitter: None,
                left_splitter: Some(400),
                current_filter: choose_option.current_filter().cloned().unwrap_or_default(),
                choose_option,
                filters: combo_box::State::new(filters),
                response,
            },
            Command::perform(update_dir_infos("."), Message::RequestNextDirs),
        )
    }

    fn namespace(&self) -> String {
        String::from("Iced Filechooser")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RequestNextDirs((dirs, pathbuf)) => {
                if is_samedir(self.dir.current_dir(), &pathbuf) {
                    self.dir.append_infos(dirs);
                    self.dir.set_end();
                }
                Command::none()
            }
            Message::RequestEnter(path) => {
                self.dir = DirUnit::enter(&path.clone());
                Command::perform(update_dir_infos(path), Message::RequestNextDirs)
            }
            Message::RequestShowHide(showhide) => {
                self.showhide = showhide;
                Command::none()
            }
            Message::RequestShowImage(showimage) => {
                self.preview_big_image = showimage;
                Command::none()
            }
            Message::RequestMultiSelect((checked, file_path)) => {
                if checked {
                    if !self.is_multi_filechooser() {
                        self.selected_paths.clear();
                    }
                    if self.selected_paths.contains(&file_path) {
                        return Command::none();
                    }
                    self.selected_paths.push(file_path);
                } else {
                    let Some(index) = self.selected_paths.iter().position(|p| *p == file_path)
                    else {
                        return Command::none();
                    };
                    self.selected_paths.remove(index);
                }
                Command::none()
            }
            Message::RequestSelect(file_path) => {
                if self.current_selected.clone().is_some_and(|p| {
                    p.canonicalize().unwrap().as_os_str()
                        == file_path.canonicalize().unwrap().as_os_str()
                }) {
                    self.current_selected = None;
                } else {
                    self.current_selected = Some(file_path.clone());
                }
                if !self.is_multi_filechooser() {
                    self.selected_paths.clear();
                }
                if self.selected_paths.contains(&file_path) {
                    return Command::none();
                }
                self.selected_paths.push(file_path.clone());
                Command::none()
            }
            Message::SearchPatternCachedChanged(pattern) => {
                self.dir.set_cache_pattern(&pattern);
                Command::none()
            }
            Message::SearchPatternChanged => {
                self.dir.set_pattern();
                Command::none()
            }
            Message::RequestAdjustRightSplitter(right_size) => {
                self.right_splitter = Some(right_size);
                Command::none()
            }
            Message::RequestAdjustLeftSplitter(left_size) => {
                self.left_splitter = Some(left_size);
                Command::none()
            }
            Message::Cancel => Command::single(Action::Window(WindowAction::Close(Id::MAIN))),
            Message::Confirm => {
                println!("eee");
                //let mut response = self.response.lock().unwrap();
                //*response = PortalResponse::Success(SelectedFiles {
                //    uris: self
                //        .selected_paths
                //        .iter()
                //        .map(|p| url::Url::from_file_path(p))
                //        .flatten()
                //        .collect(),
                //    ..Default::default()
                //});
                //println!("fff");
                //drop(response);
                Command::single(Action::Window(WindowAction::Close(Id::MAIN)))
            }
            Message::FilterChanged(filter) => {
                self.current_filter = filter;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.main_view()
    }
}

impl ShanaFileChooser {
    fn is_directory(&self) -> bool {
        self.choose_option.is_directory()
    }
    fn is_multi_filechooser(&self) -> bool {
        self.choose_option.is_multi_filechooser()
    }

    fn filter_box(&self) -> Element<Message> {
        combo_box(
            &self.filters,
            "set filter",
            Some(&self.current_filter),
            Message::FilterChanged,
        )
        .into()
    }

    fn left_view(&self) -> Element<Message> {
        let mut column = Column::new().spacing(2.);
        column = column.push(Space::with_height(10.));
        column = column.push(
            container(
                text(&self.display_name)
                    .shaping(text::Shaping::Advanced)
                    .size(20.)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    }),
            )
            .width(Length::Fill)
            .center_x(),
        );
        column = column.push(Space::with_height(10.));
        for p in self.selected_paths.iter() {
            let rp = std::fs::canonicalize(p).unwrap();
            let name = rp.to_str().unwrap();
            column = column.push(
                checkbox(name, true)
                    .on_toggle(|_| Message::RequestMultiSelect((false, p.clone())))
                    .text_size(20.),
            );
        }
        column![
            scrollable(row![Space::with_width(10.), column, Space::with_width(10.)])
                .height(Length::Fill)
                .height(Length::Fill),
            self.filter_box()
        ]
        .into()
    }
    fn main_view(&self) -> Element<Message> {
        Split::new(
            self.left_view(),
            self.dir.view(
                self.showhide,
                self.preview_big_image,
                self.right_splitter.as_ref(),
                self.current_selected.as_ref(),
                self.is_directory(),
                &self.selected_paths,
                &self.current_filter,
            ),
            self.left_splitter,
            split::Axis::Vertical,
            Message::RequestAdjustLeftSplitter,
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
