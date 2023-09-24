use rouge1::*;

fn main() {
    println!("Hello, world!");

    let mut player = Player{ x: 0, y: 0 };
    let mut game = Game{player: player, width: 20, height: 10};
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
