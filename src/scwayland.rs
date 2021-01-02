use std::{
    vec,
    fs,
    path::Path,
};

// Scans through cat /sys/class/drm/*/modes looking for used resolutions.
// TODO: perhaps we should/could stop at the first mode file that has data in it.
pub fn get_screen_resolution() -> Option<Vec<String>> {

    let mut resolutions = vec![];

    let paths = fs::read_dir("/sys/class/drm/").ok()?;    
    
    for path in paths {
        let path = path.ok()?;
        let metadata = path.metadata().ok()?;
        if metadata.is_file() {
            continue;
        }
        let mode_file = format!("/sys/class/drm/{}/modes", path.file_name().to_string_lossy());
        let mode_file_path = Path::new(&mode_file);
        if !mode_file_path.exists() {
            continue;
        }
        // We have already determined the file exists, so we shouldn't have any 
        // surprises when opening it.
        let mode_file = fs::read_to_string(mode_file).ok()?;
        if mode_file.is_empty() {
            continue;
        }

        for resolution in mode_file.lines() {
            let x_position = match resolution.find('x') {
                Some(pos) => pos,
                None => continue // This arm *should* be unreachable, I think
            };
            
            // Given a string like "1366x768", we want to return "768p". 

            //                                                        + 1 to skip the 'x' itself
            let (_, height) = resolution.split_at(x_position + 1);
            
            resolutions.push(format!("{}p", height));
        }
    }

    if resolutions.is_empty() {
        None
    } else {
        Some(resolutions)
    }
}