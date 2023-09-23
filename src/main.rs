
struct Player {
    x : usize,
    y : usize
}

fn main() {
    println!("Hello, world!");

    let mut player = Player{ x: 0, y: 0 };
    loop {

        let mut state = [[0u8; 4]; 6];
        state[player.y][player.x] = 1;

        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        match line.strip_suffix("\n").unwrap() {
            "w" => player.y-=1,
            "a" => player.x-=1,
            "s" => player.y+=1,
            "d" => player.x+=1,
            "q" => break,
            _ => println!("?????")
        }

    }

}
