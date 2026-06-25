/*
The struct that holds information about the playlist
 */
use serde::{ Serialize, Deserialize };
use crate::utils::save_utils::{
    SaveUtils,
    app_directory_path,
    LoadError,
    SaveError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistData {
    version: String, // app version the playlist was created under
    name: String,
    list: Vec<String>
}

impl PlaylistData {
    pub fn version(&self) -> String {
        self.version.clone()
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }
    
    pub fn list(&self) -> Vec<String> {
        self.list.clone()
    }
}

impl SaveUtils<PlaylistData> for PlaylistData {
    fn path() -> std::path::PathBuf {
        let mut path = app_directory_path();

        path.push("/playlists/test.json");
        path
    }
    
    async fn load() -> Result<PlaylistData, LoadError> {
        let content = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }
    
    async fn save(&self) -> Result<(), SaveError> {
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