use std::path::PathBuf;
use iced_video_player::Video;

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::tempdir;
//     use std::env;
//     use std::path::Path;
//     // tests for load_video_file function
//     // TODO create tests once the load_video_file supports Result/error handling
//     
// }