use getch::Getch;

use rouge1::*;
use rouge1::Direction::{East, North, South, West};

fn main() {
    let mut game = Game::new(10, 20);
    let getch = Getch::new();
    loop {
        let mut state = game.draw();

        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        let key = getch.getch().unwrap() as char;
        println!("{}", key);
        let direction = match key {
            'w' => (North),
            'a' => (West),
            's' => (South),
            'd' => (East),
            'q' => break,
            _ => continue
        };

        game.movePlayer(direction);
    }
}
