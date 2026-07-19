//! # Модуль для будущего вывода информации об игре.
//!
//! Содержит модуль [`score`] для хранения счета.
//! Полностью `private`

mod score;

use crate::game::snake::SnakeDirection;

struct _Info{ //TODO
    score: i32,
    direction: SnakeDirection,
}
