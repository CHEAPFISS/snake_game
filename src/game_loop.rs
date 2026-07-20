//! # Game Loop для игры в змейку.
//!
//! Имеет в себе функции для запуска и создание игрового цикла.

use crate::event::{Event as EventType, EventHandler};
use crate::game::{Game, snake::SnakeDirection};
use crate::tui::Tui;
use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

/// Структура имеющая в себе все что нужно для управления игровым циклом.
pub struct GameLoop {
    game: Game,
    running: bool,
    event_handler: EventHandler,
    tui: Tui,
}

impl GameLoop {
    /// Создает новый экземпляр [`GameLoop`].
    pub fn new(game: Game, event_handler: EventHandler, tui: Tui) -> Self {
        Self {
            game,
            running: true,
            event_handler,
            tui,
        }
    }

    /// Запускает игровой цикл.
    pub fn run(&mut self) -> Result<()> {
        self.tui.enter()?;

        while self.running {
            self.tui.draw(&mut self.game)?;

            match self.event_handler.next()?{
                EventType::Key(e) => self.input_handler(e)?,
                EventType::Tick => self.update()?,
                _ => {}
            }
        }
        self.tui.exit()?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {

        Ok(())
    }

    /// Обновляет состояние игры в зависимости от нажатой клавиши.
    fn input_handler(&mut self, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left => self.game.snake.change_direction(SnakeDirection::Left(-1)),
            KeyCode::Right => self.game.snake.change_direction(SnakeDirection::Right(1)),
            KeyCode::Up => self.game.snake.change_direction(SnakeDirection::Up(1)),
            KeyCode::Down => self.game.snake.change_direction(SnakeDirection::Down(-1)),
            _ => {}
        }
        Ok(())
    }
}
