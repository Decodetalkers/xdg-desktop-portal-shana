use iced::widget::svg;

use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum IconKey {
    Text,
    Dir,
    Path(String),
}
const TEXT_IMAGE: &[u8] = include_bytes!("../../resources/text-plain.svg");

const DIR_IMAGE: &[u8] = include_bytes!("../../resources/inode-directory.svg");

static ICON_CACHE: LazyLock<Arc<RwLock<HashMap<IconKey, svg::Handle>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

pub fn get_icon_handle(key: IconKey) -> svg::Handle {
    let icon_cache = ICON_CACHE.read().unwrap();
    if icon_cache.contains_key(&key) {
        return icon_cache.get(&key).unwrap().clone();
    }
    drop(icon_cache);
    let mut icon_cache = ICON_CACHE.write().unwrap();
    if let IconKey::Path(ref path) = key {
        if let Ok(mem) = std::fs::read(path) {
            let handle = svg::Handle::from_memory(mem);
            icon_cache.insert(key, handle.clone());
            return handle;
        };
        return svg::Handle::from_path(path);
    }
    if let IconKey::Text = key {
        let text_handle = svg::Handle::from_memory(TEXT_IMAGE);
        icon_cache.insert(key, text_handle.clone());
        return text_handle;
    }

    let dir_handle = svg::Handle::from_memory(DIR_IMAGE);
    icon_cache.insert(key, dir_handle.clone());
    dir_handle
}
