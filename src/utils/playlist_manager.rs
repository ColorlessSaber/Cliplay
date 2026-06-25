/*
Holds the methods and functions related to CRUD commands for single playlist.
*/

pub struct PlaylistManager {
    playlist_name: String,
    playlist: Vec<String>,
    index: usize, // Keeps track of where in the current playlist we are at.
}

impl PlaylistManager {
    pub fn new() -> Self {
        Self {
            playlist_name: String::new(),
            playlist: Vec::new(),
            index: 0,
        }
    }

    pub fn is_playlist_empty(&self) -> bool {
        self.playlist.is_empty()
    }

    pub fn pull_first_file_from_playlist(&mut self) -> Option<&String> {
        let file = self.playlist.get(0);
        file
    }

    pub fn pull_current_index_file_from_playlist(&mut self) -> Option<&String> {
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

    pub fn load_playlist(&mut self, list: Vec<String>) {
        self.playlist = list;
        self.index = 0;
    }

    pub fn play_single_video_file(&mut self, file_path: String) {
        self.playlist = vec![file_path];
        self.index = 0;
    }
}