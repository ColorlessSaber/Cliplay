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
    fn path(&self) -> std::path::PathBuf {
        let path = app_directory_path();

        path.join(APP_SETTINGS_FILE_NAME)
    }

    async fn load(&self) -> Result<AppSettings, LoadError> {
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