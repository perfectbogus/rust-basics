use crossterm::terminal;
use std::io;

struct Character {
    symbol: char,
    x: u16,
    y: u16,
}

impl Character {
    fn new(x: u16, y: u16) -> Self {
        Character {
            x,
            y,
            symbol: '*',
        }
    }
}

fn main() -> io::Result<()>{
    terminal::enable_raw_mode()?;

    let (width, height) = terminal::size()?;
    let center_x = width / 2;
    let center_y = height / 2;
    let mut character = Character::new(center_x, center_y);
    println!("Terminal: {}x{}, Center: ({}, {})", width, height, center_x, center_y);
    println!("{}", character.symbol);

    terminal::disable_raw_mode()?;
    Ok(())
}
