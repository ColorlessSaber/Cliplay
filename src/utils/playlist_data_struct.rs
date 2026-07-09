/*
The struct that holds information about the playlist
 */
use serde::{ Serialize, Deserialize };
use crate::utils::io_utils::{
    app_directory_path,
    LoadError,
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
    pub fn path(file_name: &String) -> std::path::PathBuf {
        let path = app_directory_path();

        path.join(PLAYLIST_FOLDER)
            .join(format!("{:}.json", file_name))
    }
    
    pub async fn load(playlist_name: &String) -> Result<PlaylistData, LoadError> {
        let content = tokio::fs::read_to_string(Self::path(playlist_name))
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