/*
Holds the enums and "utilities" for saving information.
 */

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
    // if it does not exist then defaults to the current working directory.
    //
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

pub trait SaveUtils<T> {
    fn path() -> std::path::PathBuf;

    async fn load() -> Result<T, LoadError>;

    async fn save(&self) -> Result<(), SaveError>;
}