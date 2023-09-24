struct Player {
    pub x : usize,
    pub y : usize,
}

pub struct Game {
    player: Player,
    width: usize,
    height: usize,
}

impl Game {

    pub fn new(width: usize, height: usize) -> Game {Game{player: Player{x: 0, y:0}, width, height}}
    pub fn draw(self: &Self) -> Vec<Vec<u32>> {
        let mut result = Vec::new();
        for i in 0..self.height {
            result.push(vec![0; self.width]);
        }
        result[self.player.y][self.player.x] = 1;
        return result;
    }

    pub fn movePlayer(self: &mut Self, direction: Direction) {
        match direction {
            Direction::North => self.player.y-=1,
            Direction::West => self.player.x-=1,
            Direction::South => self.player.y+=1,
            Direction::East => self.player.x+=1,
        }
    }

}

pub enum Direction {
    North, West, South, East
}
