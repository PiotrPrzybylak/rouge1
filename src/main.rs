use getch::Getch;

use rouge1::*;
use rouge1::Direction::{East, North, South, West};

mod console;

fn main() {

    let mut game = Game::new(50, 20);
    loop {
        console::clear_screen();
        let state = game.draw();
        console::draw_screen(&state);

        let direction = match console::read_key() {
            'w' => North,
            'a' => West,
            's' => South,
            'd' => East,
            'q' => break,
            _ => continue
        };

        game.move_player(direction);
    }
}


