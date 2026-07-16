/*
The data struct and methods for the settings.json file
 */
use serde::{Serialize, Deserialize};
use crate::utils::io_utils::{
    app_directory_path::app_directory_path,
    LoadError,
    SaveError,
    APP_SETTINGS_FILE_NAME
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub version: String, // version of the software
    pub skip_forward_value: usize,
    pub skip_backward_value: usize,
}

impl Default for AppSettings {
    fn default() -> Self { // Just for the record, this is the default save file
        Self {
            version: "0.1".to_string(),
            skip_forward_value: 10,
            skip_backward_value: 10,
        }
    }
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

    pub fn load() -> Result<AppSettings, LoadError> {
        let content = std::fs::read_to_string(Self::path()).map_err(|_| LoadError::File)?;

        serde_json::from_str(&content).map_err(|_| LoadError::Format)
    }

    pub fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();
        std::fs::write(path, json.as_bytes()).map_err(|_| SaveError::Write)?;

        Ok(())
    }
}