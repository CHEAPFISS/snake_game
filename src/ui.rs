//! # UI (User Interface) для игры в змейку.

use crate::{prelude::Game, ui};

use ratatui::{prelude::*, text::ToLine, widgets::{Block, BorderType, Paragraph}};

pub(crate) fn get_chunks(size: Rect) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Percentage(100),
            Constraint::Min(5),
        ])
        .split(size);
    [chunks[0], chunks[1], chunks[2]]
}


pub(crate) fn render(game: &mut Game, frame: &mut Frame) {

    let chunks = ui::get_chunks(frame.area());
    frame.render_widget(Block::bordered()
        .border_style(Style::new().fg(Color::Cyan))
        .border_type(BorderType::Double)
        .title(game.name.to_line()
            .style(Style::new()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center)
        , chunks[1]);
    frame.render_widget(&game.snake, chunks[1]);
    frame.render_widget(
        Paragraph::new(format!("Now game area is x: {},  y: {}", game.game_area.width, game.game_area.height))
            .add_modifier(Modifier::BOLD)
            .alignment(Alignment::Center),
        chunks[0]);

}
