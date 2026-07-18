use crate::event::{self, Event as EventType, EventHandler};
use crate::game::Game;
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
    /// Main struct for game loop what contains all what need for 
    /// clean work
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
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            _ => {}
        }
        Ok(())
    }
}
