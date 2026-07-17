mod snake;

use snake::Snake;
#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub area_size: (u16, u16),
    pub snake: Snake,
}

impl Game {
    pub fn new(
        name: String,
        area_size: (u16, u16),
        snake_head_position: (i32, i32),
        snake_head_symbol: char,
    ) -> Self {
        Self {
            name,
            area_size,
            snake: Snake::new(snake_head_position, snake_head_symbol),
        }
    }
}
