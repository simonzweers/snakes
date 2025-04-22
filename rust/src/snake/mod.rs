use std::collections::LinkedList;

use rand::Rng;

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
    pub food: Position<i32>,
    pub snake_nodes: LinkedList<Position<i32>>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut ret = GameState {
            active: true,
            head_direction: Position { x: 1, y: 0 },
            head_pos: Position { x: 5, y: 5 },
            food: Position {
                x: FIELD_WIDTH as i32 / 2,
                y: FIELD_HEIGHT as i32 / 2,
            },
            snake_nodes: LinkedList::new(),
        };
        ret.snake_nodes.push_back(ret.head_direction.clone());

        return ret;
    }

    fn new_food(&mut self) {
        self.food.x = rand::thread_rng().gen_range(0..FIELD_WIDTH).into();
        self.food.y = rand::thread_rng().gen_range(0..FIELD_HEIGHT).into();
    }

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

    fn head_on_food(&mut self) -> bool {
        return (self.head_pos.x == self.food.x) && (self.head_pos.y == self.food.y);
    }

    pub fn propagate(&mut self) {
        self.move_snakehead();
        self.snake_nodes.push_front(self.head_pos.clone());
        if self.head_on_food() {
            self.new_food();
        } else {
            self.snake_nodes.pop_back();
        }
    }

    pub fn check_gamestate(&mut self) {
        if (self.head_pos.x >= 0 && self.head_pos.x < FIELD_WIDTH.into())
            && (self.head_pos.y >= 0 && self.head_pos.y < FIELD_HEIGHT.into())
        {
            return;
        } else {
            self.active = false;
        }
    }

    pub fn move_snakehead(&mut self) {
        self.head_pos.x += self.head_direction.x;
        self.head_pos.y += self.head_direction.y;
    }
}

// TODO: Add constructor method to avoid unnescesary public data types
#[derive(Clone)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub fn hello_snake() {
    println!("Hello Snake!")
}
