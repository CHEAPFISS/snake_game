pub mod snake;
pub mod render;
pub mod info;

use snake::Snake;
use ratatui::{macros::ratatui_core::terminal, prelude::*};

use crate::SnakeParts;

pub struct Game {
    pub game_area: Rect,
    pub(crate) score: i32,
    pub name: String,
    pub snake: Snake,
}

impl Game {
    pub fn new(terminal_size: Size, mut snake: Snake, name: String) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Percentage(100),
                Constraint::Min(5),
            ])
            .split(Rect::from(terminal_size));

        if let Some(SnakeParts::Head(_, coords)) = snake.snake_body.front_mut(){
            coords.0 = chunks[1].width as i32 / 2;
            coords.1 = chunks[1].height as i32 / 2;
        }

        Self {
            game_area: chunks[1],
            score: 0,
            name,
            snake,
        }
    }
}
