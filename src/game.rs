//! # Модуль который требуется для хранения данных о игре.
//!
//! Создает дочерние модули: [`snake`], [`render`], [`info`]
//!
//! Требуется ТОЛЬКО для создания, хранения и чтения данных игры.

pub(crate) mod snake;
pub(crate) mod render;
pub(crate) mod info;
pub(crate) mod menu;

use snake::Snake;
use ratatui::prelude::*;
use crate::{game::menu::Menu, ui};
use snake::SnakeParts;
use std::collections::HashMap;

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
    ///Нынешнее состояние игры
    pub(crate) game_state: GameState,
    /// Меню игры.
    pub menus: HashMap<GameState, Menu>,

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,
    GameOver,
    Pause,
    AppQuit
}

impl Game {
    /// Создает новый экземпляр [`Game`].
    ///
    /// Требует размер открытого терминала типа [`Size`], экземпляр [`Snake`] и название игры типа [`String`].
    ///
    /// Кроме создания экземпляра [`Game`], `new` ставит голову [`Snake`] в центр `game_area`.
    ///
    /// # Returns
    ///
    /// Новый экземпляр [`Game`].
    pub fn new(terminal_size: Size, mut snake: Snake, name: String, menus: HashMap<GameState, Menu>) -> Self {
        let chunks = ui::get_chunks(Rect::from(terminal_size));

        center_snake(&mut snake, &chunks[1]);

        Self {
            game_state: GameState::Running,
            game_area: chunks[1],
            _score: 0,
            name,
            snake,
            menus
        }
    }
    pub(crate) fn resize_game_area(&mut self, ui_game_area: Rect) {
        self.game_area = ui_game_area;
    }
    pub(crate) fn snake_death(&mut self) {
        let (x, y) = self.snake.get_head_pos();
        if !(1..(self.game_area.width - 1) as i32).contains(&x) ||
        !(1..(self.game_area.height - 1) as i32).contains(&y) {
            self.game_state = GameState::GameOver;
        }
    }
    pub(crate) fn app_quit(&self) -> bool {
        self.game_state == GameState::AppQuit
    }

    pub fn restart(&mut self){
        self.snake = Snake::default_snake();
        center_snake(&mut self.snake, &self.game_area);
        self.game_state = GameState::Running;
    }

}

fn center_snake(snake: &mut Snake, game_area: &Rect) {
    if let Some(SnakeParts::Head(x, y)) = snake.snake_body.front_mut() {
        *x = game_area.width as i32 / 2;
        *y = game_area.height as i32 / 2;
    }
}
