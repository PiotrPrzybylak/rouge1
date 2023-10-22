use rand::random;

struct Player {
    x: usize,
    y: usize,
}

pub trait Game {
    fn draw(self: &Self) -> Vec<Vec<char>>;
    fn move_player(self: Box<Self>, direction: Direction) -> Box<dyn Game>;
}

pub struct MutableGame {
    player: Player,
    width: usize,
    height: usize,
    enemies: Vec<Enemy>,
}

struct Enemy {
    x: usize,
    y: usize,
}

pub struct ImmutableGame {
    player: Player,
    width: usize,
    height: usize,
}

impl MutableGame {
    pub fn new(width: usize, height: usize) -> Box<dyn Game> { Box::new(MutableGame { player: Player { x: 0, y: 0 }, width, height, enemies: vec![Enemy{x:1, y:0},Enemy{x:2, y:0}, Enemy{x:3, y:0}]}) }
    fn move_enemies(&mut self) {
        for enemy in &mut self.enemies.iter_mut() {
            let rnd : usize = random();
            let rnd = rnd % 4;
            static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
            let direction = &DIRECTIONS[rnd];
            match direction {
                Direction::North => if enemy.y > 0 { enemy.y -= 1 },
                Direction::West => if enemy.x > 0 { enemy.x -= 1 },
                Direction::South => if enemy.y < self.height - 1 { enemy.y += 1 },
                Direction::East => if enemy.x < self.width - 1 { enemy.x += 1 },
            }
        }
    }
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
        for enemy in self.enemies.iter() {
            result[enemy.y][enemy.x] = 'X';
        }
        return result;
    }

    fn move_player(mut self: Box<Self>, direction: Direction) -> Box<dyn Game> {
        match direction {
            Direction::North => if self.player.y > 0 { self.player.y -= 1 },
            Direction::West => if self.player.x > 0 { self.player.x -= 1 },
            Direction::South => if self.player.y < self.height - 1 { self.player.y += 1 },
            Direction::East => if self.player.x < self.width - 1 { self.player.x += 1 },
        }
        self.move_enemies();
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
