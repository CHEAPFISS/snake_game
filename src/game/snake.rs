//! # Модуль содержащий структуру и логику [`Snake`]
//!
//! Абсолютно все функции `private`, кроме `new` и `default_snake`
//!
//! # Examples
//!
//! ```
//! # use snake_game::prelude::*;
//! let snake = Snake::default_snake();
//! ```
//! или же с заданными символами
//! ```
//! # use snake_game::prelude::*;
//! let snake = Snake::new('*', 'o', 'o');
//! ```

use std::{collections::{HashMap, VecDeque}};

/// Структура [`Snake`] представляет собой змейку в игре.
pub struct Snake {
    pub(crate) snake_body: VecDeque<SnakeParts>,
    _symbols: HashMap<SnakeSymbols, char>, //TODO
    direction: SnakeDirection,
}
/// Хранит перечесление для ключей в HashMap
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SnakeSymbols {
    Head,
    Body,
    Tail,
}
/// Хранит перечесления для хранения частей тела в VecDeque<T>
pub(crate) enum SnakeParts {
    Head(char, (i32, i32)),
    _Body(char, (i32, i32)),
    _Tail(char, (i32, i32)),
}

/// Хранит перечесления определения направления движения змейки
#[derive(Clone, Copy,PartialEq, Eq)]
pub(crate) enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Snake {
    /// Создает новый экземпляр змейки с заданными символами головы, тела и хвоста. Ставит напрвление `Right`.
    pub fn new(snake_head_symbol: char, snake_body_symbol: char, snake_tail_symbol: char) -> Self {
        let mut snake_body = VecDeque::new();
        snake_body.push_front(SnakeParts::Head(snake_head_symbol, (0,0)));
        let mut _symbols = HashMap::new();
        _symbols.insert(SnakeSymbols::Head, snake_head_symbol);
        _symbols.insert(SnakeSymbols::Body, snake_body_symbol);
        _symbols.insert(SnakeSymbols::Tail, snake_tail_symbol);

        Self {
            snake_body,
            _symbols,
            direction: SnakeDirection::Right,
        }
    }
    /// Возвращает новый экземпляр змейки с символами по умолчанию ('@', '-', '*').
    pub fn default_snake() -> Self {
        Self::new('@', '-', '*')
    }
    /// Возвращает позицию головы змейки.
    pub(crate) fn _get_head_pos(&self) -> (i32, i32) { //TODO
        match self.snake_body.front(){
            Some(SnakeParts::Head(_, pos)) => *pos,
            _ => panic!("error: first must be head!"),
        }
    }

    pub(crate) fn iter_parts(&self) -> impl Iterator<Item = (char, (i32, i32))> + '_ {
        self.snake_body.iter().map(|part|{
            match part {
                SnakeParts::Head(c, pos) => (*c, *pos),
                SnakeParts::_Body(c, pos) => (*c, *pos),
                SnakeParts::_Tail(c, pos) => (*c, *pos),
            }
        })
    }
    /// Возвращает `true`, если направление [`SnakeDirection`] противоположно текущему направлению.
    fn opposite_direction(&self, direction: SnakeDirection) -> bool {
        match direction
        {
            SnakeDirection::Left => self.direction == SnakeDirection::Right,
            SnakeDirection::Right => self.direction == SnakeDirection::Left,
            SnakeDirection::Up => self.direction == SnakeDirection::Down,
            SnakeDirection::Down => self.direction == SnakeDirection::Up,
        }
    }
    /// Изменяет направление змейки на заданное [`SnakeDirection`], если оно не противоположно текущему.
    pub(crate) fn change_direction(&mut self, direction: SnakeDirection) {
        if !self.opposite_direction(direction){
            self.direction = direction;
        }
    }
}
