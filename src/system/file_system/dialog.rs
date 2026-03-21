use rfd::FileDialog;
use std::path::PathBuf;

pub fn open_file() -> Option<PathBuf> {
    FileDialog::new().pick_file()
}

pub fn open_files() -> Option<Vec<PathBuf>> {
    FileDialog::new().pick_files()
}

pub fn save_file() -> Option<PathBuf> {
    FileDialog::new().save_file()
}

pub fn pick_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}
