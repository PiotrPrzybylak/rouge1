use std::ops::{Deref, DerefMut};

use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let game = ImmutableGame::new(50, 20);
    let binding = game.move_player(East);
    let mut game1 = &binding;
    let console = Console::new();
    loop {
        let state = game1.draw();
        console.draw_screen(&state);

        let direction = match console.read_key() {
            'w' => North,
            'a' => West,
            's' => South,
            'd' => East,
            'q' => break,
            _ => continue
        };

        let game2 = game1.move_player(direction);
        game1 = &mut game2.deref_mut();
    }
}


