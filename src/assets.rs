// this file is only compile on desktop targets

use directories_next::ProjectDirs;
use std::path::PathBuf;

// constants for ProjectDirs
const PDIR_QUALIFIER : &str = "";
const PDIR_ORGANIZATION : &str = "Sylvain Tertois";
const PDIR_PROJNAME : &str = "HDL Register Wizard";
#[cfg(all(unix, not(debug_assertions)))]
const XDG_PREFIX : & str = "hdlregisterwizard";

/// path used for the settings and state save file
pub fn data_file_path() -> Option<PathBuf> {
    match ProjectDirs::from(PDIR_QUALIFIER, PDIR_ORGANIZATION, PDIR_PROJNAME) {
        Some(proj) => Some(proj.config_dir().to_path_buf()),
        _ => None,
    }
}

// in debug build, just look in the src folder
#[cfg(debug_assertions)]
pub fn find_asset(rel_path : &str) -> Option<std::path::PathBuf> {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.pop();
        exe_path.pop();
        exe_path.push("src");
        exe_path.push(rel_path);

        if exe_path.exists() {
            Some(exe_path)
        } else {
            None
        }
    } else {
        None
    }
}

// on unix, look first in a local folder, then system data folders, using the xdg crate
#[cfg(all(unix,not(debug_assertions)))]
pub fn find_asset(rel_path : &str) -> Option<std::path::PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(XDG_PREFIX).unwrap();

    xdg_dirs.find_data_file(rel_path)
}

// on windows, look first in a local folder, then in the executable folder
#[cfg(all(windows,not(debug_assertions)))]
pub fn find_asset(rel_path : &str) -> Option<std::path::PathBuf> {
    if let Some(proj) = ProjectDirs::from(PDIR_QUALIFIER, PDIR_ORGANIZATION, PDIR_PROJNAME) {
        let mut proj_data_path = proj.data_dir().to_path_buf();
        proj_data_path.push(rel_path);
        if proj_data_path.exists() {
            return Some(proj_data_path);
        }
    }

    // not found in the local folder, try executable path
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.push(rel_path);
        if exe_path.exists() {
            return Some(exe_path);
        }
    }
    None
}

// on MacOS.... no just kidding, who does hdl development on MacOS? lol
