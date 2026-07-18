use std::{collections::{HashMap, VecDeque}};
use crate::game;


pub struct Snake {
    pub(crate) snake_body: VecDeque<SnakeParts>,
    symbols: HashMap<SnakeSymbols, char>,
    direction: SnakeDirection,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SnakeSymbols{
    Head,
    Body,
    Tail,
}
pub enum SnakeParts {
    Head(char, (i32, i32)),
    Body(char, (i32, i32)),
    Tail(char, (i32, i32)),
}

#[derive(Clone, Copy,PartialEq, Eq)]
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
        let mut symbols = HashMap::new();
        symbols.insert(SnakeSymbols::Head, snake_head_symbol);
        symbols.insert(SnakeSymbols::Body, snake_body_symbol);
        symbols.insert(SnakeSymbols::Tail, snake_tail_symbol);

        Self {
            snake_body,
            symbols,
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
    ///Возвращает `true`, если направление `direction` противоположно текущему направлению.
    fn opposite_direction(&self, direction: SnakeDirection) -> bool {
        match direction
        {
            SnakeDirection::Left => self.direction == SnakeDirection::Right,
            SnakeDirection::Right => self.direction == SnakeDirection::Left,
            SnakeDirection::Up => self.direction == SnakeDirection::Down,
            SnakeDirection::Down => self.direction == SnakeDirection::Up,
        }
    }
    pub(crate) fn change_direction(&mut self, direction: SnakeDirection) {
        if !self.opposite_direction(direction){
            self.direction = direction;
        }
    }
}
impl SnakeParts {

}
