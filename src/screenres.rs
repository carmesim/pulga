use std::{fs, path::PathBuf, vec};

/// Scans through cat /sys/class/drm/*/modes looking for used resolutions.
/// Should work on both X11 and Wayland.
pub fn get_screen_resolution() -> Option<Vec<String>> {
    let mut resolutions = vec![];

    // Read all entries  "/sys/class/drm/"
    for entry in fs::read_dir("/sys/class/drm/").ok()? {
        let entry = entry.ok()?;

        let metadata = entry.metadata().ok()?;
        if metadata.is_file() {
            continue;
        }

        // Path '/sys/class/drm/{entry}/modes'
        let file_path = PathBuf::from("/sys/class/drm/")
            .join(entry.file_name())
            .join("/modes");

        let file_text = match fs::read_to_string(&file_path) {
            Ok(file_text) if !file_text.is_empty() => file_text,
            // Ignore errors and empty files
            _ => continue,
        };

        for resolution in file_text.lines() {
            // Given a string like "1366x768", we want to return "768p".
            match resolution.split('x').nth(1) {
                Some(resolution_height) => resolutions.push(format!("{}p", resolution_height)),
                // TODO: Is this unreacheable?
                None => continue,
            }
        }
    }

    match resolutions.is_empty() {
        true => None,
        false => Some(resolutions),
    }
}
