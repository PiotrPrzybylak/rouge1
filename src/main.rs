use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let game = Arc::new(Mutex::new(Option::Some( MutableGame::new2(50, 20))));
    let console = Console::new();
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(100));
            // game.move_player(South);
        }
    });
    loop {
        let mut game1 = game.lock().unwrap();
        // let mut g2 = game1.as_mut();
        let state = game1.as_ref().unwrap().draw();
        console.draw_screen( &state);

        let direction = match console.read_key() {
            'w' => North,
            'a' => West,
            's' => South,
            'd' => East,
            'q' => break,
            _ => continue
        };


        // game1.game = None;
        let a = game1.take().unwrap();
        // let option = Some(MutableGame::new2(50, 20).move_player2(direction));
        // game1.game = option;
        game1.replace(a.move_player2(direction));
        // let g123 = game1.game.take().unwrap();
        // game1.game = Some(g123.move_player2(direction));

    }
}


