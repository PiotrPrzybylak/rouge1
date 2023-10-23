use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let mut game = MutableGame::new(50, 20);
    let mutex = Mutex::new(game);
    let console = Console::new();
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(100));
            // game.move_player(South);
        }
    });
    loop {
        let mut game1 = mutex.lock().unwrap();
        let mut g2 = game1.as_mut();
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

        g2.move_player(direction);
    }
}


