pub mod display;
pub mod gamelogic;

const FIELD_HEIGHT: u16 = 20;
const FIELD_WIDTH: u16 = 20;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct GameState {
    pub active: bool,
    pub head_direction: Position<i32>,
    pub head_pos: Position<i32>,
}

impl GameState {
    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::UP => {
                self.head_direction.x = 0;
                self.head_direction.y = -1;
            }
            Direction::DOWN => {
                self.head_direction.x = 0;
                self.head_direction.y = 1;
            }
            Direction::LEFT => {
                self.head_direction.x = -1;
                self.head_direction.y = 0;
            }
            Direction::RIGHT => {
                self.head_direction.x = 1;
                self.head_direction.y = 0;
            }
        }
    }

    pub fn move_snakehead(&mut self) {
        self.head_pos.x += self.head_direction.x;
        self.head_pos.y += self.head_direction.y;
    }
}

// TODO: Add constructor method to avoid unnescesary public data types
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub fn hello_snake() {
    println!("Hello Snake!")
}
