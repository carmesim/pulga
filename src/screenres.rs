use std::{fs, path::PathBuf, vec};

/// Scans through cat /sys/class/drm/*/modes looking for used resolutions.
/// Should work on both X11 and Wayland.
pub fn get_screen_resolution() -> Option<String> {
    let mut resolutions = vec![];

    // Read all entries  "/sys/class/drm/"
    'outer: for entry in fs::read_dir("/sys/class/drm/").ok()? {
        let entry = entry.ok()?;

        let metadata = entry.metadata().ok()?;
        if metadata.is_file() {
            continue;
        }

        // Path '/sys/class/drm/{entry}/enabled'
        let enabled_path = PathBuf::from("/sys/class/drm/")
            .join(entry.file_name())
            .join("enabled");

        // Check for the 'enabled' file
        let _enabled = match fs::read_to_string(&enabled_path) {
            Ok(file_text)
                // The `enabled` file contains either "enabled" or "disabled"
                // If the monitor is disabled then we don't have to bother reading it
                if file_text.starts_with('e') => {},
            // Ignore errors and disabled monitors
            _ => continue,
        };

        // Path '/sys/class/drm/{entry}/modes'
        let file_path = PathBuf::from("/sys/class/drm/")
            .join(entry.file_name())
            .join("modes");

        let file_text = match fs::read_to_string(&file_path) {
            Ok(file_text) if !file_text.is_empty() => file_text,
            // Ignore errors and empty files
            _ => continue,
        };

        for resolution in file_text.lines() {
            // Given a string like "1366x768", we want to return "768p".
            match resolution.split('x').nth(1) {
                Some(resolution_height) => match resolution_height.parse::<i32>() {
                    Ok(number) => {
                        resolutions.push(number);
                        continue 'outer;
                    },
                    Err(_) => unreachable!("Bad number :( bad file :("),
                },
                // TODO: Is this really unreacheable?
                None => unreachable!("There's some weird stuff inside of your files mate"),
            }
        }
    }

    resolutions.sort_unstable();

    if resolutions.is_empty() {
        None
    } else {
        Some(
            resolutions
                .into_iter()
                .rfold("".to_string(), |res, val| format!("{} {}p", res, val)),
        )
    }
}
