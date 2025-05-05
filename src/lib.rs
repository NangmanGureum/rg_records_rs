pub mod bundle;
pub mod element;
pub mod string_processing;

#[cfg(test)]
mod tests {
    use crate::{
        element::song::{self, Song, Title, sort_by_title},
        string_processing::naming::Name,
    };

    #[test]
    fn title_sort() {
        let song_01 = Song {
            id: 0,
            catagory: 0,
            title: Title {
                main: Name {
                    notation: String::from("The Apple"),
                    pronunciation: None,
                },
                sub: None,
            },
            artist: Name {
                notation: String::from("Lorem"),
                pronunciation: None,
            },
            updated_date: None,
            genre: None,
        };
        let song_02 = Song {
            id: 1,
            catagory: 0,
            title: Title {
                main: Name {
                    notation: String::from("Blueberry"),
                    pronunciation: None,
                },
                sub: None,
            },
            artist: Name {
                notation: String::from("Ipsum"),
                pronunciation: None,
            },
            updated_date: None,
            genre: None,
        };
        let song_03 = Song {
            id: 3,
            catagory: 0,
            title: Title {
                main: Name {
                    notation: String::from("Tomato"),
                    pronunciation: None,
                },
                sub: None,
            },
            artist: Name {
                notation: String::from("Dolor"),
                pronunciation: None,
            },
            updated_date: None,
            genre: None,
        };

        let mut song_list = vec![song_01.clone(), song_02.clone(), song_03.clone()];

        sort_by_title(&mut song_list, false);
        assert_eq!(song_list[0].id, song_02.id);
        assert_eq!(song_list[1].id, song_01.id);
        assert_eq!(song_list[2].id, song_03.id);

        sort_by_title(&mut song_list, true);
        assert_eq!(song_list[0].id, song_01.id);
        assert_eq!(song_list[1].id, song_02.id);
        assert_eq!(song_list[2].id, song_03.id);
    }
}
