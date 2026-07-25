//! # Модуль для обработки событий через отдельный поток.
//!
//! - Использует [`crossterm`] для получения событий.
//! - Использует [`color_eyre`] для обработки ошибок.
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

/// Перечисление событий, которые могут быть обработаны.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Terminal tick
    Tick,
    /// Mouse event
    Mouse(MouseEvent),
    /// Key press
    Key(KeyEvent),
    /// Terminal resize
    Resize(u16, u16),
}
/// Обработчик событий, который запускается в отдельном потоке.
#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    /// Канал отправки событий.
    sender: mpsc::Sender<Event>,
    /// Канал получения событий.
    receiver: mpsc::Receiver<Event>,
    /// Event Handler поток
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Создает новый экземпляр [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        // Создаем частоту обновленя TUI, поток который будет обрабатывать события и отправлять их в канал.
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();

            // Создаем поток, который будет обрабатывать события и отправлять их в канал.
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let time_out = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    //  Проверяем наличие событий и отправляем их в канал.
                    if event::poll(time_out).expect("unavle to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
                    // Если время ожидания истекло, отправляем тик событие.
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }
    /// Возвращает следующее событие из канала.
    ///
    /// # Panics
    ///
    /// Если канал закрыт.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
