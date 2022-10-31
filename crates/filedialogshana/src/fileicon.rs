slint::include_modules!();
use std::{fs, path::Path};

use slint::Image;

use crate::patterns::ICONPATTERN;

pub fn source_text(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).unwrap_or("Error".to_string())
}

pub fn source_icon(pattern: &str, path: impl AsRef<Path>) -> Image {
    let icondefault = DefaultAsserts::get(&AppWindow::new()).get_fileicon();
    if ICONPATTERN.contains(&pattern) {
        match Image::load_from_path(path.as_ref()) {
            Ok(image) => image,
            Err(_) => icondefault,
        }
    } else {
        icondefault
    }
}
#[inline]
pub fn fold_icon() -> Image {
    DefaultAsserts::get(&AppWindow::new()).get_fileicon()
}
#[inline]
pub fn file_icon() -> Image {
    DefaultAsserts::get(&AppWindow::new()).get_fileicon()
}
