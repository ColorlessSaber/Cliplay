/*
Holds the enums and "utilities" for saving information.
 */

// The static folder name(s) and file name(s) help keep things consistent
pub static PLAYLIST_FOLDER: &str = "playlists/";
pub static APP_SETTINGS_FILE_NAME: &str = "settings.json";

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

pub fn app_directory_path() -> std::path::PathBuf {
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

// The boilerplate methods for CRUD actions
pub trait SaveUtils<T> {
    fn path() -> std::path::PathBuf;

    async fn load() -> Result<T, LoadError>;

    async fn save(&self) -> Result<(), SaveError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;

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
}