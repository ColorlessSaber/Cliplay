/*
Holds the methods and functions related to CRUD commands for single playlist.
*/
use rand::seq::SliceRandom;

pub struct PlaylistManager {
    playlist_name: String,
    playlist: Vec<String>,
    shuffle_order: Vec<usize>, // used for holding shuffle order of playlist
    index: usize, // Keeps track of where in the current playlist we are at.
}

impl PlaylistManager {
    pub fn new() -> Self {
        Self {
            playlist_name: String::new(),
            playlist: Vec::new(),
            shuffle_order: Vec::new(),
            index: 0,
        }
    }

    pub fn generate_shuffle_order(&mut self) {
        // minus one for .len() counts with 1
        let mut temp_shuffle_order: Vec<i32> = (0..=(self.playlist.len()-1) as i32).collect();
        temp_shuffle_order.shuffle(&mut rand::rng());

        // Change the vector from i32 to usize for playlist.len() and playlist.get()
        // only support usize.
        self.shuffle_order = temp_shuffle_order
            .iter()
            .map(|&x| x as usize)
            .collect();
    }

    pub fn clear_shuffle_order(&mut self) {
        self.shuffle_order.clear();
    }

    pub fn playlist_name(&self) -> &String {
        &self.playlist_name
    }

    pub fn extract_file_name(&self) -> Option<String> {
        self.playlist.get(self.index).map(|path| {
            let path = std::path::Path::new(path);

            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(String::from)
                .unwrap_or_else(|| "Unknown".to_string())
        })
    }

    pub fn is_playlist_empty(&self) -> bool {
        self.playlist.is_empty()
    }

    pub fn pull_first_file_from_playlist(&self) -> Option<&String> {
        self.playlist.get(0)
    }

    pub fn pull_current_index_file_from_playlist(&self) -> Option<&String> {
        self.playlist.get(self.index)
    }

    pub fn next_file_in_playlist(&mut self, loop_all: bool, is_shuffle_on: bool) -> Option<&String> {
        // if the index is greater than the length of the playlist, check to see if repeat_all is
        // true. If so, repeat the playlist; if not, return None to indicate we have reached the
        // end of the playlist

        match is_shuffle_on {
            true => {
                // Depending on the state of the shuffle_order vector and loop all:
                // If the shuffle_order is empty and loop all if on, create a new shuffle_order.
                // if the shuffle_order is empty and loop all is off, return none
                if self.shuffle_order.is_empty() && loop_all {
                    println!("create a new shuffle order");
                } else if self.shuffle_order.is_empty() && !loop_all {
                    return None
                }

                // want to keep track of the current video so if user turns off shuffle
                // it starts off where the current index is at.
                self.index = self.shuffle_order.remove(0);
                self.playlist.get(self.index)
            },
            false => {
                // the default option when shuffle is off. Increment the index, when it's greater
                // than playlist length either reset index or return none, depending on if
                // loop all is set to true.
                self.index += 1;
                if self.index > self.playlist.len()-1 { // minus one for .len() counts with 1
                    if loop_all {
                        self.index = 0;
                    } else {
                        return None
                    }
                }

                self.playlist.get(self.index)

            }
        }
    }

    pub fn previous_file_in_playlist(&mut self) -> Option<&String> {
        // regardless if the loop button is set to repeat all, circle round to the end of the
        // playlist upon reaching index of zero.
        if self.index == 0 {
            self.index = self.playlist.len()-1; // minus one for .len() counts with 1
        } else {
            self.index -= 1;
        }

        self.playlist.get(self.index)
    }

    pub fn load_playlist(&mut self, list: Vec<String>, playlist_name: String) {
        self.playlist = list;
        self.playlist_name = playlist_name;
        self.index = 0;
    }

    pub fn load_single_video_file(&mut self, file_path: String) {
        self.playlist = vec![file_path];
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_playlist_is_empty() {
        let manager = PlaylistManager::new();
        assert!(manager.is_playlist_empty());
    }

    #[test]
    fn test_pull_first_file_from_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 0,
        };
        
        assert_eq!(playlist_manager.pull_first_file_from_playlist().unwrap(), "test/video_0.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_pull_current_index_file_from_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 1,
        };

        assert!(playlist_manager.pull_current_index_file_from_playlist().is_some(), "should have successfully pulled first file");
        assert_eq!(playlist_manager.pull_current_index_file_from_playlist().unwrap(), "test/video_1.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_next_file_in_playlist_loop_off() {
        // so it is clear, this test assumes the loop is off

        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 1,
        };

        // test to see if it returns the next file
        let video_file = playlist_manager.next_file_in_playlist(false, false);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_2.mp4", "The file pulled should have matched");

        // test that it returns none given loop is off
        let video_file = playlist_manager.next_file_in_playlist(false, false);
        assert!(video_file.is_none(), "Should have reached the end of the list");
    }

    #[test]
    fn test_next_file_in_playlist_loop_on() {
        // so it is clear, this test assumes the loop is on

        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 1,
        };

        // test to see if it returns the next file
        let video_file = playlist_manager.next_file_in_playlist(true, false);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_2.mp4", "The file pulled should have matched");

        // test that it returns none given loop is off
        let video_file = playlist_manager.next_file_in_playlist(true, false);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_0.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_previous_file_in_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 1,
        };

        // test it properly grabbed previous file
        let video_file = playlist_manager.previous_file_in_playlist();
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_0.mp4", "The file pulled should have matched");

        // test it properly loop to end of list upon reaching start of list
        let video_file = playlist_manager.previous_file_in_playlist();
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_2.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_loading_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
            shuffle_order: Vec::new(),
            index: 1,
        };
        let video_list = vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()];
        playlist_manager.load_playlist(video_list, "test".to_string());

        assert_eq!(playlist_manager.playlist, vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()], "The playlist should have matched");
        assert_eq!(playlist_manager.index, 0, "the index should have reset");
    }

    #[test]
    fn test_single_video_file() {
        let mut playlist_manager = PlaylistManager::new();
        playlist_manager.load_single_video_file("test/video_0.mp4".to_string());

        assert_eq!(playlist_manager.playlist, vec!["test/video_0.mp4"], "The playlist should have matched");
        assert_eq!(playlist_manager.index, 0, "the index should have reset");
    }
}