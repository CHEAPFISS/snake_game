#[derive(Debug)]
pub struct Snake {
    snake_head_position: (i32, i32),
    snake_body: Box<Vec<Snake>>,
    snake_head_symbol: char,
}

impl Snake {
    pub fn new(snake_head_position: (i32, i32), snake_head_symbol: char) -> Self {
        Self {
            snake_head_position,
            snake_body: Box::new(Vec::new()),
            snake_head_symbol,
        }
    }
}
