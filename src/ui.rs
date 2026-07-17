use crate::Game;
use crate::game;
use ratatui::backend::ClearType::All;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn render(game: &Game, frame: &mut Frame) {
    let area = frame.size();
    let game_area = Block::bordered()
        .border_set(symbols::border::ROUNDED)
        .style(Style::default().fg(Color::White))
        .title(Line::from(game.name.as_str()).centered())
        .bold();
    frame.render_widget(game_area, area);
}
