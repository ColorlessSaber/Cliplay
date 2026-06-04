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

    pub fn pull_first_file_from_playlist(&mut self) -> Option<&String> {
        let file = self.playlist.get(self.index);
        file
    }

    pub fn next_file_in_playlist(&mut self, repeat_all: bool) -> Option<&String> {
        // if the index is greater than the length of the playlist, check to see if repeat_all is
        // true. If so, repeat the playlist; if not, return None to indicate we have reached the
        // end of the playlist
        self.index += 1;
        if self.index > self.playlist.len()-1 { // minus one for .len() counts with 1
            if repeat_all {
                self.index = 0;
            } else {
                return None;
            }
        }

        let file = self.playlist.get(self.index);
        file
    }

    pub fn previous_file_in_playlist(&mut self) -> Option<&String> {
        // regardless if the loop button is set to repeat all, circle round to the end of the
        // playlist upon reaching index of zero.
        if self.index == 0 {
            self.index = self.playlist.len()-1; // minus one for .len() counts with 1
        } else {
            self.index -= 1;
        }

        let file = self.playlist.get(self.index);
        file
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