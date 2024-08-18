use std::fs;

use glob::glob;

pub fn get_icon(theme: &str, icon: &str) -> Option<String> {
    let Ok(mut globfrometheme) =
        glob(format!("/usr/share/icons/{theme}/**/**/{icon}.svg").as_str())
    else {
        return None;
    };
    if let Some(Ok(icon)) = globfrometheme.next() {
        return Some(
            fs::canonicalize(icon)
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
        );
    }
    let Ok(mut globfrometheme) = glob(format!("/usr/share/icons/**/**/{icon}.svg").as_str()) else {
        return None;
    };
    if let Some(Ok(icon)) = globfrometheme.next() {
        return Some(
            fs::canonicalize(icon)
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
        );
    }
    None
}
