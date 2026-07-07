/*
Holds miscellaneous functions
 */
use std::path::PathBuf;
use iced_video_player::Video;
use crate::utils::save_utils::{
    PLAYLIST_FOLDER,
    app_directory_path
};

// Creates a new video to play with provided file path
pub fn load_video_file(file_path: &str) -> Video {
    //println!("Loading video from {}", file_path); // debugging
    Video::new(
        &url::Url::from_file_path(
            PathBuf::from(file!())
                .parent()
                .unwrap()
                .join(file_path)
                .canonicalize()
                .unwrap(),
        )
            .unwrap(),
    )
        .unwrap()
}

// scans the playlist folder in application directory and return the names of the playlists
// found
#[derive(Debug)]
pub enum PlaylistFolderErrors {
    DirNotFound,
    FileNotFound,
}

pub fn currently_saved_playlists(directory_path: PathBuf) -> Result<Vec<String>, PlaylistFolderErrors> {
    let mut currently_saved_playlists: Vec<String> = vec![];
    let playlist_folder_path = directory_path.join(PLAYLIST_FOLDER);
    //println!("playlist_folder_path: {:?}", playlist_folder_path); // debugging

    for entry in std::fs::read_dir(playlist_folder_path).map_err(|_| PlaylistFolderErrors::DirNotFound)? {
        let file_path = entry.map_err(|_| PlaylistFolderErrors::FileNotFound)?.path();
        if file_path.is_file() {
            if file_path.extension() == Some(std::ffi::OsStr::new("json")) {
                currently_saved_playlists.push(file_path.file_stem().unwrap().to_string_lossy().to_string());
            }
        }
    }

    Ok(currently_saved_playlists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;

    // tests for load_video_file function
    // TODO create tests once the load_video_file supports Result/error handling

    // tests for currently_saved_playlists function
    #[test]
    fn test_checking_empty_playlist_folder() {
        // create mock app directory to be used for testing
        let temp_dir = tempdir().expect("Could not create temp dir");
        let test_path = temp_dir.path().join("test");
        unsafe { env::set_var("HOME", test_path.clone()); }

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
        unsafe { env::set_var("HOME", test_path.clone()); }

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
}