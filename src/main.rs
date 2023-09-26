use getch::Getch;

use rouge1::*;
use rouge1::Direction::{East, North, South, West};

fn main() {
    let mut game = Game::new(50, 20);
    let getch = Getch::new();
    loop {
        clear_screen();
        let state = game.draw();
        draw_screen(&state);

        let key = getch.getch().unwrap() as char;
        println!("{}", key);
        let direction = match key {
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

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn draw_screen(state: &Vec<Vec<char>>) {
    for i in state {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}
