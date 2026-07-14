/*
Functions, enums, etc. that are used to handle/do IO actions
 */
use std::path::PathBuf;
use crate::utils::app_settings_struct::AppSettings;

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

// Returns the directory path of where the application directory is
pub fn app_directory_path() -> PathBuf {
    // First checks to see if the application's data directory exists.
    // Default location for linux: /home/<user_name>/.local/share/
    //
    // if it does not exist then defaults to the current working directory.
    // Causes to return current working directory:
    // * Running on Windows or when the app isn't installed via standard paths.
    let path = if let Some(project_dirs) =
        directories::ProjectDirs::from("rs", "Iced", "Cliplay") {
        project_dirs.data_dir().to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default()
    };

    path
}

// creates the application directory along with the necessary files
pub fn create_application_directory(directory_path: PathBuf) {
    std::fs::create_dir_all(directory_path.clone()).unwrap(); // main directory
    std::fs::create_dir_all(directory_path.clone().join(PLAYLIST_FOLDER)).unwrap(); // playlist folder

    if std::fs::metadata(directory_path.join(APP_SETTINGS_FILE_NAME)).is_err() {
        let _ = AppSettings::default().save();
    }
}

// scans the playlist folder in application directory and return the names of the playlists
// found
pub fn currently_saved_playlists(directory_path: PathBuf) -> Result<Vec<String>, PlaylistError> {
    let mut currently_saved_playlists: Vec<String> = vec![];
    let playlist_folder_path = directory_path.join(PLAYLIST_FOLDER);
    //println!("playlist_folder_path: {:?}", playlist_folder_path); // debugging

    for entry in std::fs::read_dir(playlist_folder_path).map_err(|_| PlaylistError::DirNotFound)? {
        let file_path = entry.map_err(|_| PlaylistError::FileNotFound)?.path();
        if file_path.is_file() {
            if file_path.extension() == Some(std::ffi::OsStr::new("json")) {
                currently_saved_playlists.push(file_path.file_stem().unwrap().to_string_lossy().to_string());
            }
        }
    }

    Ok(currently_saved_playlists)
}

pub fn delete_selected_playlist(directory_path: PathBuf, playlist_name: String) -> Result<(), PlaylistError> {
    let playlist_path = directory_path
        .join(PLAYLIST_FOLDER)
        .join(format!("{:}.json", playlist_name));

    std::fs::remove_file(playlist_path).map_err(|_| PlaylistError::FailedToDelete)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;

    // tests for app_directory_path function
    #[test]
    fn test_app_directory_does_not_exist() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");
        unsafe { env::set_var("HOME", test_path); } // temporarily override the HOME

        // test to see directory does not exist
        let app_dir_path = app_directory_path();
        assert!(!app_dir_path.exists(), "app dir path should not exist");
        assert!(!app_dir_path.is_dir(), "app dir should not be dir");
    }

    #[test]
    fn test_app_directory_exists() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");
        unsafe { env::set_var("HOME", test_path.clone()); } // temporarily override the HOME

        // create the mock app directory
        let mock_app_dir_path = test_path.join(".local/share/cliplay");
        std::fs::create_dir_all(&mock_app_dir_path).expect("Could not create mock app dir");

        // test to see directory does exist
        let app_dir_path = app_directory_path();
        assert!(app_dir_path.exists(), "app dir path should exist");
        assert!(app_dir_path.is_dir(), "app dir should be there");
    }

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

    // tests for currently_saved_playlists function
    #[test]
    fn test_checking_empty_playlist_folder() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");

        let mock_app_dir_path = test_path.clone().join(".local/share/cliplay").join(PLAYLIST_FOLDER);
        std::fs::create_dir_all(&mock_app_dir_path).expect("Could not create mock app dir");

        let list_of_playlists = currently_saved_playlists(test_path.join(".local/share/cliplay"));
        assert!(list_of_playlists.is_ok(), "the playlist folder should have been empty");
    }

    #[test]
    fn test_finding_playlists_in_folder() {
        // create mock directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");

        let mock_app_dir_path = test_path.clone().join(".local/share/cliplay").join(PLAYLIST_FOLDER);
        std::fs::create_dir_all(&mock_app_dir_path).expect("Could not create mock app dir");
        for i in 1..=10 {
            let file = &mock_app_dir_path.join(format!("playlist_{}.json", i));
            //println!("file: {:?}", file); // debugging
            std::fs::File::create(file).unwrap();
        }

        let list_of_playlists = currently_saved_playlists(test_path.join(".local/share/cliplay"));
        assert!(list_of_playlists.is_ok(), "the playlist folder should have files in it");
        assert_eq!(list_of_playlists.unwrap().len(), 10, "there should be 10 files in folder");
    }

    // tests for delete_selected_playlist function
    #[test]
    fn test_delete_selected_playlist() {
        // create mock directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");
        let mock_app_dir_path = test_path.clone().join(".local/share/cliplay").join(PLAYLIST_FOLDER);
        std::fs::create_dir_all(&mock_app_dir_path).expect("Could not create mock app dir");
        println!("{:?}", mock_app_dir_path);

        let file = &mock_app_dir_path.join("playlist_test.json");
        std::fs::File::create(file).unwrap();

        let result = delete_selected_playlist(test_path.join(".local/share/cliplay"), "playlist_test".to_string());
        assert!(result.is_ok(), "Could not delete playlist");

        assert!(!test_path.join("playlist_test.json").exists(), "playlist test file should have been deleted");

    }
}