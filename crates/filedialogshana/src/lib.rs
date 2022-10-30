mod io_file;
slint::include_modules!();
use io_file::ShowHow;
use io_file::*;
use slint::Model;

use shanatypes::{FileFilter, SelectFunction, SelectedFiles};
use slint::VecModel;
use std::{path::Path, sync::mpsc};
// 0 is succeed , 1 is cancel , 2 is other
pub fn choose_file(messages: SelectFunction) -> (u32, SelectedFiles) {
    let (title, filters, default_filter, isfold) = match messages {
        SelectFunction::File {
            title,
            filters,
            current_filter,
        } => (title, filters, current_filter, false),
        SelectFunction::Folder {
            title,
            filters,
            current_filter,
        } => (title, filters, current_filter, true),
    };
    let current_index = match &default_filter {
        None => -1,
        Some(filter) => {
            let mut index_final = -1;
            for (index, afilter) in filters.iter().enumerate() {
                if afilter.get_name() == filter.get_name() {
                    index_final = index as i32;
                    break;
                }
            }
            index_final
        }
    };
    let defaultfiliter = match default_filter {
        Some(filiter) => Some(vec![filiter]),
        None => None,
    };
    let filters_forset = filters.clone();
    let (stdout_tx, stdout_rs) = mpsc::channel();
    let stdout_tx_cancel = stdout_tx.clone();
    let ui = AppWindow::new();
    ui.set_m_title(title.into());
    let globalfiles = GlobalFiles::get(&ui);
    globalfiles.set_filiter(
        std::rc::Rc::new(VecModel::from(
            filters
                .iter()
                .enumerate()
                .map(|(index, unit)| FilterFun {
                    filiter: unit.get_name().into(),
                    selected: current_index == index as i32,
                })
                .collect::<Vec<FilterFun>>(),
        ))
        .into(),
    );
    globalfiles.set_is_select_fold(isfold);
    globalfiles.set_current_path((*HOME).clone().into());
    globalfiles.set_current_filiter(current_index);
    globalfiles.set_left(
        std::rc::Rc::new(VecModel::from(
            get_files_from_folder((*HOME).to_string(), ShowHow::OnlyVisible, defaultfiliter)
                .into_iter()
                .map(|file| match file {
                    FileType::File {
                        name,
                        owner,
                        readable,
                        runable,
                        filepath,
                        mimetype,
                        ..
                    } => FileUnit {
                        name: name.into(),
                        owner: owner.into(),
                        is_fold: false,
                        mimetype: mimetype.into(),
                        permission: match (readable, runable) {
                            (false, false) => 0,
                            (true, false) => 1,
                            (false, true) => 2,
                            (true, true) => 3,
                        },
                        file_path: filepath.into(),
                        beselected: false,
                    },
                    FileType::Folder {
                        name,
                        owner,
                        viewable,
                        filepath,
                        ..
                    } => FileUnit {
                        name: name.into(),
                        owner: owner.into(),
                        is_fold: true,
                        mimetype: "".into(),
                        permission: if viewable { 1 } else { 0 },
                        file_path: filepath.into(),
                        beselected: false,
                    },
                })
                .collect::<Vec<FileUnit>>(),
        ))
        .into(),
    );
    let ui_handle = ui.as_weak();
    ui.on_change_superpath(move || {
        let ui = ui_handle.unwrap();
        let globalfiles = GlobalFiles::get(&ui);
        let current_path = globalfiles.get_current_path().to_string();
        let path = Path::new(&current_path);
        if let Some(path) = path.parent() {
            globalfiles.set_current_path(path.to_str().unwrap().into());
        }
    });
    let ui_handle = ui.as_weak();
    ui.on_select_file(move || {
        let ui = ui_handle.unwrap();
        let globalfiles = GlobalFiles::get(&ui);
        let left = globalfiles.get_left();
        let selected = left
            .iter()
            .filter(|unit| unit.beselected)
            .map(|unit| unit.file_path.to_string())
            .collect();
        let selectfiles = SelectedFiles::from_path(selected);
        let _ = stdout_tx.send((
            0,
            selectfiles,
        ));
        let _ = slint::quit_event_loop();
    });
    ui.on_cancel_selected(move || {
        let _ = stdout_tx_cancel.send((
            1,
            SelectedFiles {
                uris: vec![],
                choices: None,
            },
        ));
        let _ = slint::quit_event_loop();
    });
    let ui_handle = ui.as_weak();
    ui.on_change_filiter(move |howtoshow, find_path, side| {
        let ui = ui_handle.unwrap();
        let globalfiles = GlobalFiles::get(&ui);
        let filiters = globalfiles.get_filiter().to_owned();
        let showmod = match howtoshow {
            0 => ShowHow::OnlyVisible,
            1 => ShowHow::ShowHidden,
            2 => ShowHow::OnlyHidden,
            3 => ShowHow::OnlyFile,
            _ => ShowHow::OnlyFolder,
        };
        let filiter = filiters.iter().collect::<Vec<FilterFun>>();
        let toselected = std::iter::zip(filiter, filters_forset.clone())
            .filter(|(filiter, _)| filiter.selected)
            .map(|(_, filiter)| filiter)
            .collect::<Vec<FileFilter>>();
        if side == 0 {
            globalfiles.set_left(
                std::rc::Rc::new(VecModel::from(
                    get_files_from_folder(find_path.to_string(), showmod, Some(toselected))
                        .into_iter()
                        .map(|file| match file {
                            FileType::File {
                                name,
                                owner,
                                readable,
                                runable,
                                filepath,
                                mimetype,
                                ..
                            } => FileUnit {
                                name: name.into(),
                                owner: owner.into(),
                                is_fold: false,
                                mimetype: mimetype.into(),
                                permission: match (readable, runable) {
                                    (false, false) => 0,
                                    (true, false) => 1,
                                    (false, true) => 2,
                                    (true, true) => 3,
                                },
                                file_path: filepath.into(),
                                beselected: false,
                            },
                            FileType::Folder {
                                name,
                                owner,
                                viewable,
                                filepath,
                                ..
                            } => FileUnit {
                                name: name.into(),
                                owner: owner.into(),
                                is_fold: true,
                                mimetype: "".into(),
                                permission: if viewable { 1 } else { 0 },
                                file_path: filepath.into(),
                                beselected: false,
                            },
                        })
                        .collect::<Vec<FileUnit>>(),
                ))
                .into(),
            );
        } else {
            globalfiles.set_right(
                std::rc::Rc::new(VecModel::from(
                    get_files_from_folder(find_path.to_string(), showmod, Some(toselected))
                        .into_iter()
                        .map(|file| match file {
                            FileType::File {
                                name,
                                owner,
                                readable,
                                runable,
                                filepath,
                                mimetype,
                                ..
                            } => FileUnit {
                                name: name.into(),
                                owner: owner.into(),
                                is_fold: false,
                                mimetype: mimetype.into(),
                                permission: match (readable, runable) {
                                    (false, false) => 0,
                                    (true, false) => 1,
                                    (false, true) => 2,
                                    (true, true) => 3,
                                },
                                file_path: filepath.into(),
                                beselected: false,
                            },
                            FileType::Folder {
                                name,
                                owner,
                                viewable,
                                filepath,
                                ..
                            } => FileUnit {
                                name: name.into(),
                                owner: owner.into(),
                                is_fold: true,
                                mimetype: "".into(),
                                permission: if viewable { 1 } else { 0 },
                                file_path: filepath.into(),
                                beselected: false,
                            },
                        })
                        .collect::<Vec<FileUnit>>(),
                ))
                .into(),
            );
        }
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
