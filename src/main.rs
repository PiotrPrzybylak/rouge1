use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let game_mutex1 = Arc::new(Mutex::new(Option::Some( MutableGame::new2(50, 20))));
    let game_mutex2 = game_mutex1.clone();
    thread::spawn(move || {
        let console = Console::new();
        loop {
            thread::sleep(Duration::from_millis(100));
            let game = game_mutex2.lock().unwrap().take().unwrap();
            let state = game.draw();
            console.draw_screen( &state);
            let new_game = game.move_game();
            game_mutex2.lock().unwrap().replace(new_game);
        }
    });
    let console = Console::new();
    loop {
        let direction = match console.read_key() {
            'w' => North,
            'a' => West,
            's' => South,
            'd' => East,
            'q' => break,
            _ => continue
        };

        let mut game = game_mutex1.lock().unwrap();
        let state = game.as_ref().unwrap().draw();
        console.draw_screen( &state);
        let new_game = game.take().unwrap().move_player2(direction);
        game.replace(new_game);

    }
}


