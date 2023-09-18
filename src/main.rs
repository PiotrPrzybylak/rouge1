use std::mem::forget;

fn main() {
    println!("Hello, world!");


    let mut state = [[0u8; 4]; 6];
    for i in state {
        for j in i {
            print!("{}", j);
        }
        println!();
    }

}
