mod io_file;
slint::include_modules!();
use io_file::ShowHow;
use io_file::*;
use shanatypes::{SelectFunction, SelectedFiles};
use slint::VecModel;
use std::sync::mpsc;
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
                .map(|unit| unit.get_name().into())
                .collect::<Vec<slint::SharedString>>(),
        ))
        .into(),
    );
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
                    },
                })
                .collect::<Vec<FileUnit>>(),
        ))
        .into(),
    );
    ui.on_select_file(move || {
        let _ = stdout_tx.send((
            0,
            SelectedFiles {
                uris: vec![],
                choices: None,
            },
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
    ui.on_change_filiter(move |index, howtoshow| {
        let ui = ui_handle.unwrap();
        let globalfiles = GlobalFiles::get(&ui);
        let showmod = match howtoshow {
            0 => ShowHow::OnlyVisible,
            1 => ShowHow::ShowHidden,
            _ => ShowHow::OnlyHidden,
        };
        globalfiles.set_left(
            std::rc::Rc::new(VecModel::from(
                get_files_from_folder(
                    globalfiles.get_current_path().to_string(),
                    showmod,
                    Some(vec![filters_forset[index as usize].clone()]),
                )
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
                    },
                })
                .collect::<Vec<FileUnit>>(),
            ))
            .into(),
        );
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
