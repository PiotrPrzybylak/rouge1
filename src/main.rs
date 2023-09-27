use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let mut game = ImmutableGame::new(50, 20);
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

        game = game.move_player(direction);
    }
}


