use std::path::PathBuf;
use iced_video_player::Video;
use std::io;

#[derive(Debug)]
pub enum LoadVideoFileError {
    Io(io::Error),
    NotAbsolutePath(String),
}

impl From<io::Error> for LoadVideoFileError {
    fn from(e: io::Error) -> Self {
        LoadVideoFileError::Io(e)
    }
}

// Creates a new video to play with provided file path
pub fn load_video_file(file_path: &str) -> Result<Video, LoadVideoFileError> {
    //println!("Loading video from {}", file_path); // debugging
    let file_path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join(file_path)
        .canonicalize()?;

    let url = url::Url::from_file_path(file_path).map_err(|_| {
        LoadVideoFileError::NotAbsolutePath("File path is not an absolute path".to_string())
    })?;

    Video::new(&url).map_err(|err| LoadVideoFileError::Io(io::Error::new(io::ErrorKind::Other, err)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_video_file_failure_io_error() {
        let non_existent_path = "/non/existent/path";
        let foo = load_video_file(non_existent_path);
        println!("{:?}", foo);

        let result = match foo {
            Err(LoadVideoFileError::Io(_)) => true,
            _ => false,
        };
        assert!(result);
    }
}