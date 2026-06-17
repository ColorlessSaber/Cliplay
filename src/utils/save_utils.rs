/*
Holds the struct of the application save information, possible errors when saving/loading,
and methods to save/load application information.
 */
use serde::{Serialize, Deserialize };
use crate::utils::playlist_entry_data::PlaylistEntryData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedInfo {
    pub version: String, // version of the software
    pub playlists: Vec<PlaylistEntryData>,
}

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

// TODO look into having separate paths for app_data and folder to hold individual playlists.
impl SavedInfo {
    fn path() -> std::path::PathBuf {
        // Creates the file path to the "app_data.json" file which is located in the
        // application folder

        // First checks to see if the application's data directory exists.
        // if it does not exist then defaults to the current working directory.
        //
        // Causes to return current working directory:
        // * Running on Windows or when the app isn't installed via standard paths.
        let mut path = if let Some(project_dirs) =
            directories::ProjectDirs::from("rs", "Iced", "Cliplay") {
            project_dirs.data_dir().to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("app_data.json");
        path
    }

    pub async fn load() -> Result<SavedInfo, LoadError> {
        let content = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }

    pub async fn save(&self) -> Result<(), SaveError> {
        let json = serde_json::to_string(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path();

        // if the directory does not exist, create it. Else, save the data
        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::Write)?;
        }

        {
            tokio::fs::write(path, json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        Ok(())
    }
}