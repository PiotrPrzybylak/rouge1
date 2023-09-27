struct Player {
    pub x: usize,
    pub y: usize,
}

pub trait Game {
    fn draw(self: &Self) -> Vec<Vec<char>>;
    fn move_player(self: Box<Self>, direction: Direction) -> Box<dyn Game>;
}

pub struct MutableGame {
    player: Player,
    width: usize,
    height: usize,
}

pub struct ImmutableGame {
    player: Player,
    width: usize,
    height: usize,
}

impl MutableGame {
    pub fn new(width: usize, height: usize) -> Box<dyn Game> { Box::new(MutableGame { player: Player { x: 0, y: 0 }, width, height }) }
}

impl ImmutableGame {
    pub fn new(width: usize, height: usize) -> Box<dyn Game> { Box::new(ImmutableGame { player: Player { x: 0, y: 0 }, width, height }) }
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

    fn move_player(mut self: Box<Self>, direction: Direction) -> Box<dyn Game> {
        match direction {
            Direction::North => if self.player.y > 0 { self.player.y -= 1 },
            Direction::West => if self.player.x > 0 { self.player.x -= 1 },
            Direction::South => if self.player.y < self.height - 1 { self.player.y += 1 },
            Direction::East => if self.player.x < self.width - 1 { self.player.x += 1 },
        }
        self
    }
}

impl Game for ImmutableGame {
    fn draw(self: &Self) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for _ in 0..self.height {
            result.push(vec!['.'; self.width]);
        }
        result[self.player.y][self.player.x] = '@';
        return result;
    }

    fn move_player(self: Box<Self>, direction: Direction) -> Box<dyn Game> {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        match direction {
            Direction::North => if self.player.y > 0 { dy = -1 },
            Direction::West => if self.player.x > 0 { dx = -1 },
            Direction::South => if self.player.y < self.height - 1 { dy = 1 },
            Direction::East => if self.player.x < self.width - 1 { dx = 1 },
        }
        Box::new(ImmutableGame { player: Player { x: (self.player.x as i32 + dx) as usize, y: (self.player.y as i32 + dy) as usize }, width: self.width, height: self.height })
    }
}


pub enum Direction {
    North,
    West,
    South,
    East,
}
