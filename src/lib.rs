//! # Snake Game
//!
//! Мой личный мини проектик для того, чтоб понять, что нужно для игрового движка на базе [crossterm] и [ratatui].
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
//! # Panics
//!
//! - Не удалось установить panic hook.
//! - Не удалось запустить терминал.
//! - Не удалось запустить игровой цикл.

mod event;
mod game;
mod game_loop;
mod tui;
mod ui;

pub mod prelude;

pub use prelude::*;
