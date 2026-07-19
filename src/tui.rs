//! # TUI (Terminal User Interface) для игры в змейку.
//!
//! - Использует [`crossterm`] для управления терминалом.
//! - Использует [`ratatui`] для отрисовки интерфейса.
//! - Использует [`color_eyre`] для обработки ошибок.


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

/// Тип просто для сокращения CrosstermTerminal
pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;
/// Структура для управления TUI (Terminal User Interface).
pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    /// Создает новый экземпляр Tui.
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }
    /// Переходит в режим альтернативного экрана, захватывает мышь и panic_hook.
    ///
    /// # Errors
    /// Возвращает ошибку, если переход не удался.
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
    /// Возвращает терминал в исходное состояние.
    ///
    /// # Errors
    /// Возвращает ошибку, если сброс не удался.
    pub fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
    /// Выходит из режима альтернативного экрана и освобождает мышь.
    ///
    /// # Errors
    /// Возвращает ошибку, если выход не удался.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
    /// Отрисовывает весь TUI приложения через [`ui::render()`].
    ///
    /// # Errors
    /// Возвращает ошибку, если отрисовка не удалась.
    pub fn draw(&mut self, game: &mut Game) -> Result<()> {
        self.terminal.draw(|frame| ui::render(game, frame))?;
        Ok(())
    }
}
