//! # UI (User Interface) для игры в змейку.

use crate::prelude::Game;

use ratatui::{prelude::*, text::ToLine, widgets::{Block, BorderType}};


pub(crate) fn render(game: &mut Game, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Percentage(100),
            Constraint::Min(5),
        ])
        .split(frame.area());
    game.game_area = chunks[1];
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

}
