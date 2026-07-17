use color_eyre::eyre::Result;
use std::panic;

use std::io;

use ratatui::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::Game;
use crate::ui;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    /// Creates a new Tui instance.
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset terminal");
            panic_hook(panic);
        }));
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, game: &mut Game) -> Result<()> {
        self.terminal.draw(|frame| ui::render(game, frame))?;
        Ok(())
    }
}
