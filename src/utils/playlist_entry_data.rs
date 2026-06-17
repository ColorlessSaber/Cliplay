/*
The struct that holds information about the playlist
 */
use uuid::Uuid;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistEntryData {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid, // to help grabbing the correct one when loading to play or editing.
    pub name: String,
    pub list: Vec<String>
}