// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    current: (i32, i32),
    head_dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            current: (x, y),
            head_dir: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            head_dir: match self.head_dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            head_dir: match self.head_dir {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            current: match self.head_dir {
                Direction::North => (self.current.0, self.current.1 + 1),
                Direction::East => (self.current.0 + 1, self.current.1),
                Direction::South => (self.current.0, self.current.1 - 1),
                Direction::West => (self.current.0 - 1, self.current.1),
            },
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'A' => robot.advance(),
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.current
    }

    pub fn direction(&self) -> &Direction {
        &self.head_dir
    }
}
