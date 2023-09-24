pub struct Player {
    pub x : usize,
    pub y : usize,
}

pub struct Game {
    pub player: Player,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn draw(self: &Self) -> Vec<Vec<u32>> {
        let mut result = Vec::new();
        for i in 0..self.height {
            result.push(vec![0; self.width]);
        }
        result[self.player.y][self.player.x] = 1;
        return result;
    }
}
