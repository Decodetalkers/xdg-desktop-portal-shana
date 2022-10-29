mod io_file;
slint::include_modules!();
use io_file::ShowHow;
use io_file::*;
use shanatypes::SelectedFiles;
use slint::VecModel;
use std::sync::mpsc;
// 0 is succeed , 1 is cancel , 2 is other
pub fn choose_file() -> (u32, SelectedFiles) {
    let (stdout_tx, stdout_rs) = mpsc::channel();
    let stdout_tx_cancel = stdout_tx.clone();
    let ui = AppWindow::new();
    let globalfiles = GlobalFiles::get(&ui);
    globalfiles.set_left(
        std::rc::Rc::new(VecModel::from(
            get_files_from_folder((*HOME).to_string(), ShowHow::OnlyVisible)
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
