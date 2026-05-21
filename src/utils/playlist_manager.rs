/*
Holds the playlist manager struct with methods and functions related to CRUD commands for the
playlist manipulation.
*/

pub struct PlayListManager {
    playlist: Vec<String>,
    index: u8, // Keeps track of where in the current playlist we are at. Using 8-bit for not for seeing a playlist with +255 vids
}

impl Default for PlayListManager { // for testing purposes
    fn default() -> Self {
        Self {
            playlist: vec![
                "/home/admin/Videos/Misc Videos/Zenless Zone Zero/Burnice Character Demo -  A Burnice Special for the Brokenhearted    Zenless Zone Zero.mp4".to_string(),
                "/home/admin/Videos/Misc Videos/Zenless Zone Zero/Caesar Character Demo -  Calydon's Ride    Zenless Zone Zero.mp4".to_string(),
            ],
            index: 0,
        }
    }
}

impl PlayListManager {
    pub fn new() -> Self {
        Self {
            playlist: Vec::new(),
            index: 0,
        }
    }

    pub fn next_file_in_playlist(&mut self) -> Option<&String> {
        // An Option-None is when the application as reached the end of the playlist.
        let file = self.playlist.get(self.index as usize);
        self.index += 1;
        file
    }

    pub fn reset_index(&mut self) {
        self.index = 0;
    }

}