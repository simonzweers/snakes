pub mod display;
pub mod gamelogic;

const FIELD_HEIGHT: u16 = 20;
const FIELD_WIDTH: u16 = 20;

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct GameState {
    pub active: bool,
    pub head_pos_y: u16,
    pub head_pos_x: u16,
}

pub fn hello_snake() {
    println!("Hello Snake!")
}
