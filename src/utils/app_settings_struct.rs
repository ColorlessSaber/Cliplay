/*
The struct that holds user settings for the application.
 */
use serde::{Serialize, Deserialize };
use crate::utils::io_utils::{
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

impl AppSettings {
    pub fn path() -> std::path::PathBuf {
        let path = app_directory_path();

        path.join(APP_SETTINGS_FILE_NAME)
    }

    pub async fn load() -> Result<AppSettings, LoadError> {
        let content = tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path();

        {
            tokio::fs::write(path, json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        Ok(())
    }
}