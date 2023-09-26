use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let game: &mut dyn Game = &mut MutableGame::new(50, 20);
    let console = Console::new();
    loop {
        let state = game.draw();
        console.draw_screen(&state);

        let direction = match console.read_key() {
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


