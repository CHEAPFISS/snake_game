pub mod event;
pub mod game;
pub mod game_loop;
pub mod tui;
pub mod ui;

pub use game::snake::{Snake, SnakeParts};
pub use event::EventHandler;
pub use game::Game;
pub use game_loop::GameLoop;
pub use tui::Tui;
