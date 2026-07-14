// Logo Icons
pub mod logo_icons {
    pub static CLIPLAY_LOGO_GREY_ICON: &str = "icons/cliplay_logo_grey.png";
}

// Video Player Control Icons
pub mod video_player_icons {
    pub static MAIN_MENU_CLOSED_ICON: &str = "icons/main_menu_closed.png";
    pub static MAIN_MENU_OPEN_ICON: &str = "icons/main_menu_open.png";
    pub static PLAY_ICON: &str = "icons/play.png";
    pub static STOP_ICON: &str = "icons/stop.png";
    pub static PAUSE_ICON: &str = "icons/pause.png";
    pub static FORWARD_ICON: &str = "icons/forward.png";
    pub static BACKWARD_ICON: &str = "icons/backward.png";
    pub static SKIP_BACKWARD_ICON: &str = "icons/skip_backward.png";
    pub static SKIP_FORWARD_ICON: &str = "icons/skip_forward.png";
    pub static LOOP_OFF_ICON: &str = "icons/loop_off.png";
    pub static LOOP_ONE_ICON: &str = "icons/loop_single.png";
    pub static LOOP_INFINITE_ICON: &str = "icons/loop_infinite.png";
    pub static SHUFFLE_ICON: &str = "icons/shuffle.png";
    pub static VOLUME_ICON: &str = "icons/volume.png";
}

// Main Menu Icons
pub mod main_menu_icons {
    pub static SELECT_VID_FROM_COMPUTER_ICON: &str = "icons/select_single_vid.png";
    pub static PLAYLISTS_ICON: &str = "icons/playlists_icon.png";
    pub static SETTINGS_ICON: &str = "icons/settings_icon.png";
}

//  Playlist Menu Icons
pub mod playlist_menu_icons {
    pub static NEW_PLAYLIST_ICON: &str = "icons/new_playlist_icon.png";
    pub mod entry { // icons for playlist entry
        pub static EDIT_PLAYLIST_ICON: &str = "icons/edit_playlist_icon.png";
        pub static DELETE_PLAYLIST_ICON: &str = "icons/delete_playlist_icon.png";
        pub static PLAY_PLAYLIST_ICON: &str = "icons/play_playlist_icon.png";
    }

    pub mod editor { // icons for the playlist editor
        pub static SELECT_VIDEO_FILE_ICON: &str = "icons/select_video_file.png";
        pub static REMOVE_VIDEO_FILE_ICON: &str = "icons/remove_video_file.png";
        pub static SAVE_ICON: &str = "icons/save_icon.png";
    }
}
