use color_eyre::eyre::Result;
use snake_game_teaching::prelude::*;
use std::io::stderr;
use std::collections::HashMap;

use ratatui::Terminal;

fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;

    let backend = ratatui::backend::CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;

    let term_size = terminal.size()?;

    let pause_menu = Menu::default("PAUSE");
    let game_over = Menu::new("GAME OVER", {
        vec![Item::restart(), Item::quit()]
    });
    let game = Game::new(
        term_size,
        Snake::default_snake(),
        String::from("Snake Game"),
        HashMap::from([
            (GameState::Pause, pause_menu),
            (GameState::GameOver, game_over),
        ]),
    );


    let event_handler = EventHandler::new(200);
    let tui = Tui::new(terminal);

    let mut game_loop = GameLoop::new(game, event_handler, tui);
    game_loop.run()?;

    Ok(())
}
