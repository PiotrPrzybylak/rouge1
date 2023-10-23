use std::{thread, time};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use rouge1::*;
use rouge1::Direction::{East, North, South, West};

use crate::console::Console;

mod console;

fn main() {
    let mut game = MutableGame::new(50, 20);
    let console = Console::new();
    let stdin_channel = spawn_stdin_channel();

    loop {
        let state = game.draw();
        console.draw_screen(&state);

        if let Ok(key) = stdin_channel.try_recv() {
            let direction = match key {
                'w' => North,
                'a' => West,
                's' => South,
                'd' => East,
                'q' => break,
                _ => continue
            };
            game = game.move_player(direction);
        }

        game = game.move_game();

        let duration = time::Duration::from_millis(10);
        thread::sleep(duration);
    }
}

fn spawn_stdin_channel() -> Receiver<char> {
    let (tx, rx) = mpsc::channel::<char>();
    let console = Console::new();
    thread::spawn(move || loop {
        tx.send(console.read_key()).unwrap();
    });
    rx
}


