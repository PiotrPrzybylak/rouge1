use getch::Getch;

pub struct Console {
    key_reader: Getch,
}

impl Console {
    pub fn new() -> Console { Console { key_reader: Getch::new() } }

    pub fn clear_screen(self: &Self) {
        print!("{esc}c", esc = 27 as char);
    }

    pub fn draw_screen(self: &Self, state: &Vec<Vec<char>>) {
        self.clear_screen();
        for i in state {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }

    pub fn read_key(self: &Self) -> char {
        self.key_reader.getch().unwrap() as char
    }
}

