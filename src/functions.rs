/*
Holds miscellaneous functions
 */
use iced_video_player::Video;

// Creates a new video to play with provided file path
pub fn new_video_to_play(file_path: &str) -> Video {
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