pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn draw_screen(state: &Vec<Vec<char>>) {
    for i in state {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}