/*
The struct that holds information about the playlist
 */
use serde::{ Serialize, Deserialize };
use crate::utils::save_utils::{
    SaveUtils,
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

impl SaveUtils<PlaylistData> for PlaylistData {
    fn path(&self) -> std::path::PathBuf {
        let path = app_directory_path();

        path.join(format!("{}{}{}", PLAYLIST_FOLDER, self.name, ".json"))
    }
    
    async fn load(&self) -> Result<PlaylistData, LoadError> {
        let content = tokio::fs::read_to_string(Self::path(&self))
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }
    
    async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path(&self);

        {
            tokio::fs::write(path, json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        Ok(())
    }
    
}