
struct Player {
    x : usize,
    y : usize,
}

struct Game {
    player: Player,
    width: usize,
    height: usize,
}

impl Game {
    fn draw(self: &Self) -> Vec<Vec<u32>> {
        let mut result = Vec::new();
        for i in 0..self.height {
            result.push(vec![0; self.width]);
        }
        return result;
    }
}

fn main() {
    println!("Hello, world!");

    let mut player = Player{ x: 0, y: 0 };
    let mut game = Game{player: player, width: 20, height: 10};
    loop {
        let mut state = game.draw();
        state[game.player.y][game.player.x] = 1;

        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        match line.strip_suffix("\n").unwrap() {
            "w" => game.player.y-=1,
            "a" => game.player.x-=1,
            "s" => game.player.y+=1,
            "d" => game.player.x+=1,
            "q" => break,
            _ => println!("?????")
        }

    }

}
