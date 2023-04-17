use std::path::PathBuf;

use lazy_static::lazy_static;
use platform_dirs::AppDirs;

include!(concat!(env!("OUT_DIR"), "/workspace.rs"));

lazy_static! {
    /// The platform-specific save directory for the app.
    pub static ref SAVE_DIR: PathBuf = {
        AppDirs::new(Some(WORKSPACE), true)
            .unwrap()
            .data_dir
            .join("saves")
    };
}

/// Returns the absolute path to a save file given its name.
pub fn get_save_file(name: &str) -> PathBuf {
    SAVE_DIR.join(format!("{name}.sav"))
}