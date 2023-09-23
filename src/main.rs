

fn main() {
    println!("Hello, world!");

    let mut position_x = 0;
    let mut position_y = 0;
    loop {

        let mut state = [[0u8; 4]; 6];
        state[position_y][position_x] = 1;

        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        match line.strip_suffix("\n").unwrap() {
            "w" => position_y-=1,
            "a" => position_x-=1,
            "s" => position_y+=1,
            "d" => position_x+=1,
            "q" => break,
            _ => println!("?????")
        }

    }

}
