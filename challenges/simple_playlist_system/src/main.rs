use std::rc::Rc;

#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    duration: u32,
}

#[derive(Debug)]
struct Playlist {
    name: String,
    songs: Vec<Rc<Song>>
}

impl Song {
    fn new(title: String, artist: String, duration: u32) -> Song {
        Self { title, artist, duration }
    }
}

impl Playlist {
    fn new(name: String) -> Playlist {
        Self { name, songs: vec![] }
    }

    fn add_song(&mut self, song: Rc<Song>) {
        self.songs.push(song)
    }

    fn get_total_duration(&self) -> u32 {
        self.songs.iter().map(|song| song.duration).sum()
    }

    fn find_songs_by_artist(&self, artist: &str) -> Vec<Rc<Song>> {
        self.songs.iter()
            .filter(|song| song.artist == artist)
            .cloned()
            .collect()
    }

    fn share_songs_with(&self, other_playlist: &mut Playlist) {
        self.songs.iter()
            .cloned()
            .for_each(|song| other_playlist.add_song(song));
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_duration() {
        let song1 = Rc::new(Song::new(
            "Song 1".to_string(),
            "Artist 1".to_string(),
            180,
        ));

        let song2 = Rc::new(Song::new(
            "Song 2".to_string(),
            "Artist 2".to_string(),
            240,
        ));

        let mut playlist = Playlist::new("My Playlist".to_string());
        playlist.add_song(Rc::clone(&song1));
        playlist.add_song(Rc::clone(&song2));

        assert_eq!(playlist.get_total_duration(), 420);
    }

    #[test]
    fn test_find_songs_by_artist() {
        let song1 = Rc::new(Song::new(
            "Song 1".to_string(),
            "Artist 1".to_string(),
            180,
        ));

        let song2 = Rc::new(Song::new(
            "Song 2".to_string(),
            "Artist 1".to_string(),
            240,
        ));

        let song3 = Rc::new(Song::new(
            "Song 3".to_string(),
            "Artist 2".to_string(),
            200
        ));

        let mut playlist = Playlist::new("My playlist".to_string());
        playlist.add_song(Rc::clone(&song1));
        playlist.add_song(Rc::clone(&song2));
        playlist.add_song(Rc::clone(&song3));

        let artist1_songs = playlist.find_songs_by_artist("Artist 1");

        assert_eq!(playlist.name, "My playlist".to_string());
        assert_eq!(artist1_songs.len(), 2);
        assert!(artist1_songs.iter().any(|song| song.title == "Song 1"));
        assert!(artist1_songs.iter().any(|song| song.title == "Song 2"));
    }

    #[test]
    fn test_share_songs() {
        let song = Rc::new(Song::new(
           "Test Song".to_string(),
           "Test Artist".to_string(),
           100
        ));

        let mut pl1 = Playlist::new("Test Playlist".to_string());
        let mut pl2 = Playlist::new("Test Playlist".to_string());

        pl1.add_song(Rc::clone(&song));
        pl1.share_songs_with(&mut pl2);

        assert_eq!(pl1.songs.len(), 1);
        assert_eq!(pl2.songs.len(), 1);
        assert_eq!(Rc::strong_count(&song), 3);
    }

}