/*
Holds miscellaneous functions
 */
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
            std::path::PathBuf::from(file!())
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
pub enum DirError {
    DirNotFound,
}

pub fn currently_saved_playlists() -> Result<Vec<String>, DirError> {
    let mut currently_saved_playlists: Vec<String> = vec![];

    let mut path = app_directory_path();
    path.push(PLAYLIST_FOLDER);

    for entry in std::fs::read_dir(path).map_err(|_| DirError::DirNotFound)? {
        let file_path = entry.map_err(|_| DirError::DirNotFound)?.path();
        if file_path.is_file() {
            if file_path.extension() == Some(std::ffi::OsStr::new("json")) {
                currently_saved_playlists.push(file_path.file_stem().unwrap().to_string_lossy().to_string());
            }
        }
    }

    Ok(currently_saved_playlists)
}