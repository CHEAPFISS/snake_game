use std::collections::VecDeque;

pub struct Snake {
    pub(crate) snake_body: VecDeque<SnakeParts>,

    snake_head_symbol: char,
    snake_body_symbol: char,
    snake_tail_symbol: char,
    pub(crate) direction: SnakeDirection,
}

pub enum SnakeParts {
    Head(char, (i32, i32)),
    Body(char, (i32, i32)),
    Tail(char, (i32, i32)),
}

pub(crate) enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Snake {
    pub(crate) fn new(snake_start_position: (i32, i32), snake_head_symbol: char, snake_body_symbol: char, snake_tail_symbol: char) -> Self {
        let mut snake_body = VecDeque::new();
        snake_body.push_front(SnakeParts::Head(snake_head_symbol, snake_start_position));

        Self {
            snake_body,
            snake_head_symbol,
            snake_body_symbol,
            snake_tail_symbol,
            direction: SnakeDirection::Right,
        }
    }
    pub fn default_snake() -> Self {
        Self::new((0,0), '@', '-', '*')
    }
    pub(crate) fn get_head_pos(&self) -> (i32, i32) {
        match self.snake_body.front(){
            Some(SnakeParts::Head(_, pos)) => *pos,
            _ => panic!("error: first must be head!"),
        }
    }
    pub(crate) fn iter_parts(&self) -> impl Iterator<Item = (char, (i32, i32))> + '_ {
        self.snake_body.iter().map(|part|{
            match part {
                SnakeParts::Head(c, pos) => (*c, *pos),
                SnakeParts::Body(c, pos) => (*c, *pos),
                SnakeParts::Tail(c, pos) => (*c, *pos),
            }
        })
    }
}
