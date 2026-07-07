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
            index: 0,
        };

        assert!(playlist_manager.pull_first_file_from_playlist().is_some(), "should have successfully pulled first file");
        assert_eq!(playlist_manager.pull_first_file_from_playlist().unwrap(), "test/video_0.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_pull_current_index_file_from_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()],
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
            index: 1,
        };

        // test to see if it returns the next file
        let video_file = playlist_manager.next_file_in_playlist(false);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_2.mp4", "The file pulled should have matched");

        // test that it returns none given loop is off
        let video_file = playlist_manager.next_file_in_playlist(false);
        assert!(video_file.is_none(), "Should have reached the end of the list");
    }

    #[test]
    fn test_next_file_in_playlist_loop_on() {
        // so it is clear, this test assumes the loop is on

        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
            index: 1,
        };

        // test to see if it returns the next file
        let video_file = playlist_manager.next_file_in_playlist(true);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_2.mp4", "The file pulled should have matched");

        // test that it returns none given loop is off
        let video_file = playlist_manager.next_file_in_playlist(true);
        assert!(video_file.is_some(), "It should have grabbed an existing file from list");
        assert_eq!(video_file.unwrap(), "test/video_0.mp4", "The file pulled should have matched");
    }

    #[test]
    fn test_previous_file_in_playlist() {
        let mut playlist_manager = PlaylistManager{
            playlist_name: String::new(),
            playlist: vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string(), "test/video_2.mp4".to_string()],
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
            index: 1,
        };
        let video_list = vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()];
        playlist_manager.load_playlist(video_list);

        assert_eq!(playlist_manager.playlist, vec!["test/video_0.mp4".to_string(), "test/video_1.mp4".to_string()], "The playlist should have matched");
        assert_eq!(playlist_manager.index, 0, "the index should have reset");
    }

    #[test]
    fn test_single_video_file() {
        let mut playlist_manager = PlaylistManager::new();
        playlist_manager.play_single_video_file("test/video_0.mp4".to_string());

        assert_eq!(playlist_manager.playlist, vec!["test/video_0.mp4"], "The playlist should have matched");
        assert_eq!(playlist_manager.index, 0, "the index should have reset");
    }
}