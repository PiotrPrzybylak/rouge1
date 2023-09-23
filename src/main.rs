use std::mem::forget;

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
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        if line.contains("q") {
            break;
        }

        if line.contains("a") {
            position_x-=1;
        }
        if line.contains("d") {
            position_x+=1;
        }

        if line.contains("w") {
            position_y-=1;
        }
        if line.contains("s") {
            position_y+=1;
        }

    }

}
