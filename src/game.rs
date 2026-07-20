//! # Модуль который требуется для хранения данных о игре.
//!
//! Создает дочерние модули: [`snake`], [`render`], [`info`]
//!
//! Требуется ТОЛЬКО для создания, хранения и чтения данных игры.

pub(crate) mod snake;
pub(crate) mod render;
pub(crate) mod info;

use snake::Snake;
use ratatui::prelude::*;

use snake::SnakeParts;



/// Структура, представляющая игру.
///
/// Включает в себя экземпляр [`Snake`], игровое поле типа [`Rect`], счетчик очков и название игры типа [`String`].
pub struct Game {
    /// Игровое поле типа [`Rect`]
    pub game_area: Rect,
    /// Счетчик очков.
    pub(crate) _score: i32,
    /// Название игры.
    pub name: String,
    /// Экземпляр [`Snake`].
    pub snake: Snake,
}

impl Game {
    /// Создает новый экземпляр [`Game`].
    ///
    /// Требует размер открытого терминала типа [`Size`], экземпляр [`Snake`] и название игры типа [`String`].
    ///
    /// Кроме создания экземпляра [`Game`], `new` ставит голову [`Snake`] в центре `game_area`.
    ///
    /// # Returns
    ///
    /// Новый экземпляр [`Game`].
    pub fn new(terminal_size: Size, mut snake: Snake, name: String) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Percentage(100),
                Constraint::Min(5),
            ])
            .split(Rect::from(terminal_size));

        if let Some(SnakeParts::Head(x, y)) = snake.snake_body.front_mut() {
            *x = chunks[1].width as i32 / 2;
            *y = chunks[1].height as i32 / 2;
        }

        Self {
            game_area: chunks[1],
            _score: 0,
            name,
            snake,
        }
    }
}
