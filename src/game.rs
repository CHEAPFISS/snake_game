pub mod snake;
pub mod render;

use snake::Snake;

pub struct Game {
    pub name: String,
    pub area_size: (u16, u16),
    pub snake: Snake,
}

impl Game {
    pub fn new(snake: Snake, name: String, area_size: (u16, u16)) -> Self {
        Self {
            name,
            area_size,
            snake,
        }
    }
}
