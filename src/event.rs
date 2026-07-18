use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Apple event
    Apple,
    /// Terminal tick
    Tick,
    /// Mouse event
    Mouse(MouseEvent),
    /// Key press
    Key(KeyEvent),
    /// Terminal resize
    Resize(u16, u16),
}
#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    /// Event sender channel
    sender: mpsc::Sender<Event>,
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    ///Event Handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    ///Make a new instance of [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let time_out = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(time_out).expect("unavle to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Resize(_, _) => continue,
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
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
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
