/*
Holds miscellaneous functions
 */
use iced_video_player::Video;

// Creates a new video to play with provided file path
pub fn load_video_file(file_path: &str) -> Video {
    println!("Loading video from {}", file_path); // debugging
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