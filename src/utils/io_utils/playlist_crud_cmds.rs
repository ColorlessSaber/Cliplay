/*
Holds CRUD commands for a playlist file and playlist folder
 */
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::utils::io_utils::{
    app_directory_path::app_directory_path,
    LoadError,
    PlaylistError,
    SaveError,
    PLAYLIST_FOLDER
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistData {
    pub software_version: String, // app version the playlist was created under
    pub name: String,
    pub list: Vec<String>
}

impl PlaylistData {
    pub fn path(file_name: &String) -> PathBuf {
        let path = app_directory_path();

        path.join(PLAYLIST_FOLDER)
            .join(format!("{:}.json", file_name))
    }
    
    pub async fn load(playlist_name: String) -> Result<PlaylistData, LoadError> {
        let content = tokio::fs::read_to_string(Self::path(&playlist_name))
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }
    
    pub async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path(&self.name);

        {
            tokio::fs::write(path, json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        Ok(())
    }
}

pub fn scan_playlist_folder(directory_path: PathBuf) -> Result<Vec<String>, PlaylistError> {
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
mod tests_playlist_crud_cmds {
    use super::*;
    use tempfile::tempdir;

    // tests for currently_saved_playlists function
    #[test]
    fn test_checking_empty_playlist_folder() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");

        let mock_app_dir_path = test_path.clone().join(".local/share/cliplay").join(PLAYLIST_FOLDER);
        std::fs::create_dir_all(&mock_app_dir_path).expect("Could not create mock app dir");

        let list_of_playlists = scan_playlist_folder(test_path.join(".local/share/cliplay"));
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
            std::fs::File::create(file).unwrap();
        }

        let list_of_playlists = scan_playlist_folder(test_path.join(".local/share/cliplay"));
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

        let file = &mock_app_dir_path.join("playlist_test.json");
        std::fs::File::create(file).unwrap();

        let result = delete_selected_playlist(test_path.join(".local/share/cliplay"), "playlist_test".to_string());
        assert!(result.is_ok(), "Could not delete playlist");

        assert!(!test_path.join("playlist_test.json").exists(), "playlist test file should have been deleted");

    }
}