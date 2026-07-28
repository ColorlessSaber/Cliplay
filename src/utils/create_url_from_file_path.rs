use std::path::PathBuf;
use std::io;

#[derive(Debug)]
pub enum LoadVideoFileError {
    Io(io::Error),
    NotAbsolutePath,
}

impl From<io::Error> for LoadVideoFileError {
    fn from(e: io::Error) -> Self {
        LoadVideoFileError::Io(e)
    }
}

pub fn create_url_from_file_path(file_path: &str) -> Result<url::Url, LoadVideoFileError> {
    //println!("Loading video from {}", file_path); // debugging
    let file_path = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join(file_path)
        .canonicalize()?;

    let url = url::Url::from_file_path(file_path).map_err(|_| {
        LoadVideoFileError::NotAbsolutePath
    })?;
    
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_video_file_failure_io_error() {
        let non_existent_path = "/non/existent/path";
        let foo = create_url_from_file_path(non_existent_path);

        let result = match foo {
            Err(LoadVideoFileError::Io(_)) => true,
            _ => false,
        };
        assert!(result);
    }
}