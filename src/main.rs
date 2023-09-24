use rouge1::*;
use rouge1::Direction::{East, North, South, West};

fn main() {

    let mut game = Game::new(10, 20);
    loop {
        let mut state = game.draw();

        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        let direction = match line.strip_suffix("\n").unwrap() {
            "w" => (North),
            "a" => (West),
            "s" => (South),
            "d" => (East),
            "q" => break,
            _ => continue
        };

        game.movePlayer(direction);

    }

}
