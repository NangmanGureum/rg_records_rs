use crate::string_processing::naming::Name;

/// Struct for describe a song
#[derive(Debug, Clone)]
pub struct Song {
    /// ID of the song
    pub id: usize,
    /// Category of the song. It depends on the game
    pub catagory: usize,
    /// A title of the song. See `Title` struct
    pub title: Title,
    /// An artist of the song
    pub artist: Name,
    /// A date of when the song updated. It follows RFC 3339
    pub updated_date: Option<String>,
    /// A genre of the song
    pub genre: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Title {
    pub main: Name,
    pub sub: Option<Name>,
}

pub fn sort_by_title(songs: &mut Vec<Song>, except_the: bool) {
    songs.sort_by(|a, b| {
        let a_title = &a.title.main.notation;
        let b_title = &b.title.main.notation;

        let mut a_first_char = 'a';
        let mut b_first_char = 'a';

        // 'The' exception
        if except_the {
            if a_title.to_lowercase().starts_with("the ") {
                a_first_char = a_title.to_lowercase().chars().nth(4).unwrap();
            } else {
                a_first_char = a_title.to_lowercase().chars().nth(0).unwrap();
            }

            if b_title.to_lowercase().starts_with("the ") {
                b_first_char = b_title.to_lowercase().chars().nth(4).unwrap();
            } else {
                b_first_char = b_title.to_lowercase().chars().nth(0).unwrap();
            }
        } else {
            a_first_char = a_title.to_lowercase().chars().nth(0).unwrap();
            b_first_char = b_title.to_lowercase().chars().nth(0).unwrap();
        }

        a_first_char.cmp(&b_first_char)
    });
}
