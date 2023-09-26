struct Player {
    pub x: usize,
    pub y: usize,
}

pub trait Game {
    fn draw(self: &Self) -> Vec<Vec<char>>;
    fn move_player(self: &mut Self, direction: Direction);
}

pub struct MutableGame {
    player: Player,
    width: usize,
    height: usize,
}

impl MutableGame {
    pub fn new(width: usize, height: usize) -> MutableGame { MutableGame { player: Player { x: 0, y: 0 }, width, height } }

}
impl Game for MutableGame {
    fn draw(self: &Self) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for _ in 0..self.height {
            result.push(vec!['.'; self.width]);
        }
        result[self.player.y][self.player.x] = '@';
        return result;
    }

    fn move_player(self: &mut Self, direction: Direction) {
        match direction {
            Direction::North => if self.player.y > 0 { self.player.y -= 1 },
            Direction::West => if self.player.x > 0 { self.player.x -= 1 },
            Direction::South => if self.player.y < self.height - 1 { self.player.y += 1 },
            Direction::East => if self.player.x < self.width - 1 { self.player.x += 1 },
        }
    }
}

pub enum Direction {
    North,
    West,
    South,
    East,
}
