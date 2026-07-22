//! # Game Loop для игры в змейку.
//!
//! Имеет в себе функции для запуска и создание игрового цикла.

use crate::event::{Event, EventHandler};
use crate::game::GameState;
use crate::game::{Game, snake::SnakeDirection};
use crate::tui::Tui;
use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::layout::{Rect, Size};
use crate::ui;

/// Структура имеющая в себе все что нужно для управления игровым циклом.
pub struct GameLoop {
    game: Game,
    event_handler: EventHandler,
    tui: Tui,
}

impl GameLoop {
    /// Создает новый экземпляр [`GameLoop`].
    pub fn new(game: Game, event_handler: EventHandler, tui: Tui) -> Self {
        Self {
            game,
            event_handler,
            tui,
        }
    }

    /// Запускает игровой цикл.
    pub fn run(&mut self) -> Result<()> {
        self.tui.enter()?;

        while !self.game.app_quit() {
            self.tui.draw(&mut self.game)?;

            let event = self.event_handler.next()?;

            match self.game.game_state {
                GameState::GameOver | GameState::Pause => {
                    if let Event::Key(e) = event {

                    }
                },
                GameState::Runnning => {
                    match event {
                        Event::Key(e) => self.input_handler_game(e)?,
                        Event::Tick => self.update()?,
                        Event::Resize(w, h) =>{
                            let chunks = ui::get_chunks(Rect::from(Size::new(w, h)));
                            self.game.resize_game_area(chunks[1]);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        self.tui.exit()?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.game.snake.move_snake();
        self.game.snake_death();
        Ok(())
    }

    /// Обновляет состояние игры в зависимости от нажатой клавиши.
    fn input_handler_game(&mut self, event: KeyEvent) -> Result<()> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.game.game_state = GameState::Pause, //TODO Сделать паузу и меню выбора
            KeyCode::Left => self.game.snake.change_direction(SnakeDirection::Left),
            KeyCode::Right => self.game.snake.change_direction(SnakeDirection::Right),
            KeyCode::Up => self.game.snake.change_direction(SnakeDirection::Up),
            KeyCode::Down => self.game.snake.change_direction(SnakeDirection::Down),
            _ => {}
        }
        Ok(())
    }
    fn input_handler_pause(&mut self, event: KeyEvent) -> Result<()> {
        match event.code {

            _ => {}
        }
        Ok(())
    }


}
