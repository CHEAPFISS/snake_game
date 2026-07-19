//! # Game Loop для игры в змейку.
//!
//! Имеет в себе функции для запуска и создание игрового цикла.
//!
//! Ниже приведен пример для запуска игры:
//!
//! # Example
//! ```rust, no_run
//! use color_eyre::eyre::Result;
//! use snake_game::{EventHandler, Game, GameLoop, Snake, Tui};
//! use std::io::stderr;
//!
//! use ratatui::Terminal;
//!
//! fn main() -> Result<(), color_eyre::eyre::Report> {
//!     color_eyre::install()?;
//!
//!     let backend = ratatui::backend::CrosstermBackend::new(stderr());
//!     let terminal = Terminal::new(backend)?;
//!     let term_size = terminal.size()?;
//!
//!     let game = Game::new(
//!         term_size,
//!         Snake::default_snake(),
//!         String::from("Snake Game"),
//!     );
//!
//!     let event_handler = EventHandler::new(100);
//!     let tui = Tui::new(terminal);
//!
//!     let mut game_loop = GameLoop::new(game, event_handler, tui);
//!     game_loop.run()?;
//!     Ok(())
//! }
//! ```
//!
//! # Panics
//!
//! - Не удалось установить panic hook.
//! - Не удалось запустить терминал.
//! - Не удалось запустить игровой цикл.

use crate::event::{Event as EventType, EventHandler};
use crate::game::{Game, snake::SnakeDirection};
use crate::tui::Tui;
use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub struct GameLoop {
    game: Game,
    running: bool,
    event_handler: EventHandler,
    tui: Tui,
}

impl GameLoop {

    pub fn new(game: Game, event_handler: EventHandler, tui: Tui) -> Self {
        Self {
            game,
            running: true,
            event_handler,
            tui,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.tui.enter()?;

        while self.running {
            self.tui.draw(&mut self.game)?;

            if let Ok(EventType::Key(e)) = self.event_handler.next() {
                self.update(e)?;
            }
        }
        self.tui.exit()?;
        Ok(())
    }

    fn update(&mut self, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left => self.game.snake.change_direction(SnakeDirection::Left),
            KeyCode::Right => self.game.snake.change_direction(SnakeDirection::Right),
            KeyCode::Up => self.game.snake.change_direction(SnakeDirection::Up),
            KeyCode::Down => self.game.snake.change_direction(SnakeDirection::Down),
            _ => {}
        }
        Ok(())
    }
}
