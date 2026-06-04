/*
Holds the playlist manager struct with methods and functions related to CRUD commands for the
playlist manipulation.
*/

pub struct PlayListManager {
    playlist: Vec<String>,
    index: usize, // Keeps track of where in the current playlist we are at.
}

impl PlayListManager {
    pub fn new() -> Self {
        Self {
            playlist: Vec::new(),
            index: 0,
        }
    }

    pub fn next_file_in_playlist(&mut self, repeat_all: bool) -> Option<&String> {
        // Returns none upon reaching the end of the playlist
        loop {
            let file = self.playlist.get(self.index);
            self.index += 1;
            if file.is_none() {
                if repeat_all {
                    self.index = 0;
                } else {
                    return None;
                }
            } else {
                return file;
            }
        }
    }

    pub fn load_playlist(&mut self) {
        self.playlist = vec![
            "/home/admin/Videos/Misc Videos/Zenless Zone Zero/Caesar Character Demo -  Calydon's Ride    Zenless Zone Zero.mp4".to_string(),
            "/home/admin/Videos/Misc Videos/Zenless Zone Zero/Hoshimi Miyabi Character Demo -  Everlasting Training    Zenless Zone Zero.mkv".to_string(),
            "/home/admin/Videos/Misc Videos/Zenless Zone Zero/ZZZ WIT Studio Animation.mkv".to_string(),
            "/home/admin/Videos/Misc Videos/Zenless Zone Zero/Burnice Character Demo -  A Burnice Special for the Brokenhearted    Zenless Zone Zero.mp4".to_string(),
        ];
        self.index = 0;
    }

    pub fn add_file_to_playlist(&mut self, file_path: String) {
        self.playlist.push(file_path);
    }

    pub fn clear_playlist(&mut self) {
        self.playlist.clear();
        self.index = 0;
    }


}