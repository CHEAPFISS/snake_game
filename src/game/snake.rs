//! # Модуль содержащий структуру и логику [`Snake`]
//!
//! Абсолютно все функции `private`, кроме `new` и `default_snake`
//!
//! # Examples
//!
//! ```
//! # use snake_game_teaching::prelude::*;
//! let snake = Snake::default_snake();
//! ```
//! или же с заданными символами
//! ```
//! # use snake_game_teaching::prelude::*;
//! let snake = Snake::new('*', 'o', 'o');
//! ```

use std::{collections::{HashMap, VecDeque}};

/// Структура [`Snake`] представляет собой змейку в игре.
pub struct Snake {
    pub(crate) snake_body: VecDeque<SnakeParts>,
    pub(crate) symbols: HashMap<SnakeSymbols, char>, //TODO
    direction: Hvdir,
}

/// Хранит направление движения змейки как горизонтальное или вертикальное направление.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Hvdir {
    Horizontal(i8),
    Vertical(i8),
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
    Head(i32, i32),
    Body(i32, i32),
    Tail(i32, i32),
}

/// Хранит перечесления определения направления движения змейки
#[derive(Clone, Copy,PartialEq, Eq)]
pub(crate) enum SnakeDirection {
    Left(i8),
    Right(i8),
    Up(i8),
    Down(i8),
}

impl Snake {
    /// Создает новый экземпляр змейки с заданными символами головы, тела и хвоста. Ставит напрвление `Right`.
    pub fn new(snake_head_symbol: char, snake_body_symbol: char, snake_tail_symbol: char) -> Self {
        let mut snake_body = VecDeque::new();
        snake_body.push_front(SnakeParts::Head(0,0));
        let mut symbols = HashMap::new();
        symbols.insert(SnakeSymbols::Head, snake_head_symbol);
        symbols.insert(SnakeSymbols::Body, snake_body_symbol);
        symbols.insert(SnakeSymbols::Tail, snake_tail_symbol);

        Self {
            snake_body,
            symbols,
            direction: SnakeDirection::Right(1),
        }
    }

    pub(crate) fn move_snake(&mut self){


    }

    /// Возвращает новый экземпляр змейки с символами по умолчанию ('@', '-', '*').
    pub fn default_snake() -> Self {
        Self::new('@', '-', '*')
    }
    /// Возвращает позицию головы змейки.
    pub(crate) fn get_head_pos(&self) -> (i32, i32) {
        match self.snake_body.front(){
            Some(SnakeParts::Head(x, y)) => (*x, *y),
            _ => panic!("error: first must be head!"),
        }
    }

    pub(crate) fn iter_parts(&self) -> impl Iterator<Item = (SnakeSymbols, (i32, i32))> + '_ {
        let symbols = &self.symbols;
        self.snake_body.iter().map(|part|{
            match part {
                SnakeParts::Head(x, y) => (SnakeSymbols::Head, (*x, *y)),
                SnakeParts::Body(x, y) => (SnakeSymbols::Body, (*x, *y)),
                SnakeParts::Tail(x, y) => (SnakeSymbols::Tail, (*x, *y)),
            }
        })
    }
    /// Возвращает `true`, если направление [`SnakeDirection`] противоположно текущему направлению.
    fn opposite_direction(&self, direction: SnakeDirection) -> bool {
        match direction
        {
            SnakeDirection::Left(v) => self.direction == SnakeDirection::Right(-v),
            SnakeDirection::Right(v) => self.direction == SnakeDirection::Left(-v),
            SnakeDirection::Up(v) => self.direction == SnakeDirection::Down(-v),
            SnakeDirection::Down(v) => self.direction == SnakeDirection::Up(-v),
        }
    }
    /// Изменяет направление змейки на заданное [`SnakeDirection`], если оно не противоположно текущему.
    pub(crate) fn change_direction(&mut self, direction: SnakeDirection) {
        if !self.opposite_direction(direction){
            self.direction = direction;
        }
    }
}
