/*
The struct that holds user settings for the application.
 */

use serde::{Serialize, Deserialize };
use crate::utils::save_utils::{
    SaveUtils,
    app_directory_path,
    LoadError,
    SaveError,
    APP_SETTINGS_FILE_NAME
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    version: String, // version of the software
}

impl AppSettings {
    pub fn version(&self) -> String {
        self.version.clone()
    }
}

impl SaveUtils<AppSettings> for AppSettings {
    fn path() -> std::path::PathBuf {
        let mut path = app_directory_path();

        path.push(format!("/{}", APP_SETTINGS_FILE_NAME));
        path
    }

    async fn load() -> Result<AppSettings, LoadError> {
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