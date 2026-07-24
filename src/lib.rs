//! # Snake Game
//!
//! Мой личный мини проектик для того, чтоб понять, что нужно для игрового движка на базе [crossterm] и [ratatui].
//!
//! Ниже приведен пример для запуска игры:
//!
//! # Example
//! ```rust, no_run
//! use color_eyre::eyre::Result;
//! use snake_game_teaching::prelude::*;
//! use std::io::stderr;
//! use std::collections::HashMap;

//! use ratatui::Terminal;

//! fn main() -> Result<(), color_eyre::eyre::Report> {
//!     color_eyre::install()?;

//!     let backend = ratatui::backend::CrosstermBackend::new(stderr());
//!     let terminal = Terminal::new(backend)?;

//!     let term_size = terminal.size()?;

//!     let pause_menu = Menu::default("PAUSE");
//!     let game_over = Menu::new("GAME OVER", {
//!         vec![Item::restart(), Item::quit()]
//!     });
//!     let game = Game::new(
//!         term_size,
//!         Snake::default_snake(),
//!         String::from("Snake Game"),
//!         HashMap::from([
//!             (GameState::Pause, pause_menu),
//!             (GameState::GameOver, game_over),
//!         ]),
//!     );


//!     let event_handler = EventHandler::new(200);
//!     let tui = Tui::new(terminal);

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
