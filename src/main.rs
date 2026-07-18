use color_eyre::eyre::Result;
use snake_game::{EventHandler, Game, GameLoop, Snake, Tui};
use std::io::stderr;

use ratatui::Terminal;

fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;

    let backend = ratatui::backend::CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;

    let term_size = terminal.size()?;

    let game = Game::new(
        term_size,
        Snake::default_snake(),
        String::from("Snake Game"),
    );


    let event_handler = EventHandler::new(100);
    let tui = Tui::new(terminal);

    let mut game_loop = GameLoop::new(game, event_handler, tui);
    game_loop.run()?;

    Ok(())
}
