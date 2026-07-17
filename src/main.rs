use color_eyre::eyre::Result;
use snake_game::{EventHandler, Game, GameLoop, Tui};
use std::{
    io::{Write, stderr, stdout},
    rc::Rc,
};

use ratatui::Terminal;

fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;

    let game = Game::new(
        String::from("Snake Game"),
        crossterm::terminal::size().expect("Error: Failed to get terminal size"),
        (0, 0),
        '@',
    );

    let backend = ratatui::backend::CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;
    let event_handler = EventHandler::new(500);
    let tui = Tui::new(terminal);

    let mut game_loop = GameLoop::new(game, event_handler, tui);
    game_loop.run()?;

    Ok(())
}
