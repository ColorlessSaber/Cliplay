/*
Functions, enums, etc. that are used to handle/do IO actions
 */
pub mod app_settings_data_struct;
pub mod playlist_crud_cmds;
pub mod app_directory_path;
pub mod create_url_from_file_path;

use std::path::PathBuf;
use app_settings_data_struct::AppSettings;

// The static folder name(s) and file name(s) help keep things consistent
pub static PLAYLIST_FOLDER: &str = "playlists";
pub static APP_SETTINGS_FILE_NAME: &str = "settings.json";

// Enum(s) to handle the different errors
#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    Write,
    Format,
}

#[derive(Debug)]
pub enum PlaylistError {
    DirNotFound,
    FileNotFound,
    FailedToDelete,
}

// creates the application directory along with the necessary files
pub fn create_application_directory(directory_path: PathBuf) {
    std::fs::create_dir_all(directory_path.clone()).unwrap(); // main directory
    std::fs::create_dir_all(directory_path.clone().join(PLAYLIST_FOLDER)).unwrap(); // playlist folder

    if std::fs::metadata(directory_path.join(APP_SETTINGS_FILE_NAME)).is_err() {
        let _ = AppSettings::default().save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // tests for create_application_directory
    #[test]
    fn test_creation_of_app_directory() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().unwrap();
        create_application_directory(temp_dir.path().join(".local/share/cliplay"));

        // test to see that application directory has been created
        let mock_app_dir_path = temp_dir.path().join(".local/share/cliplay");
        assert!(mock_app_dir_path.exists(), "Could not create mock app dir");

        // test to see if playlist folder exists
        let mock_playlist_folder_path = temp_dir.path().join(".local/share/cliplay").join(PLAYLIST_FOLDER);
        assert!(mock_playlist_folder_path.exists(), "playlist folder should have been created");

    }
}