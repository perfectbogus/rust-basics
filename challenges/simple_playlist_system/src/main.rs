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

    }
}

fn main() {
    println!("Hello, world!");
}
