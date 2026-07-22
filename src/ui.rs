//! # UI (User Interface) для игры в змейку.

use std::vec;

use crate::{prelude::Game, ui};

use ratatui::{prelude::*, text::ToLine, widgets::{Block, BorderType, Paragraph}};

pub(crate) fn get_chunks(size: Rect) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Fill(100),
            Constraint::Min(3),
        ])
        .split(size);
    [chunks[0], chunks[1], chunks[2]]
}


pub(crate) fn render(game: &mut Game, frame: &mut Frame) {

    let chunks = ui::get_chunks(frame.area());

    let info_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
        )
        .split(chunks[0]);


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

    let coord = vec![
        Line::from(
            format!(
                "Width: {}, Height: {}",
                game.game_area.width,
                game.game_area.height
            )
        ),
        Line::from(" "),
        Line::from(
            format!(
                "X: {}, Y: {}",
                game.snake.get_head_pos().0,
                game.snake.get_head_pos().1
            )
        ),
    ];

    frame.render_widget(
        Paragraph::new(coord)
            .add_modifier(Modifier::BOLD)
            .alignment(Alignment::Center)
            .block(Block::bordered()
                .border_style(Style::new().fg(Color::Cyan))
                .border_type(BorderType::Plain)
                .title(Line::from("Area Size")
                    .alignment(Alignment::Center)
                    .style(
                        Style::new()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::LightBlue)))
                .title_bottom(Line::from("Player Coordinates")
                    .alignment(Alignment::Center)
                    .style(
                        Style::new().fg(Color::Magenta)
                    )
                )
                ),
        info_area[2]);



}
