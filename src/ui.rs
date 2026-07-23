//! # UI (User Interface) для игры в змейку.

use std::vec;

use crate::{Item, Menu, game::GameState, prelude::Game, ui};

use ratatui_core::layout::{Layout, Flex};
use ratatui::prelude::*;
use ratatui::text::ToLine;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};

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
                Constraint::Min(25),
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

    if game.game_state == GameState::Pause{
        render_popup_block(frame, &game.menus[&GameState::Pause], &chunks[1]);
    }

}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn menu_buttons_place_rect(chunk: Rect, items: &[Item]) -> Vec<Rect> {
    let rect = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::SpaceBetween)
        .constraints(
            items.iter().map(|item|{
                Constraint::Min(item.name.len() as u16)
            })
        )
        .split(chunk);
    rect.to_vec()
}

fn render_popup_block(frame: &mut Frame, menu: &Menu, chunk: &Rect){
    let popup_area = centered_rect(40, 30, *chunk);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Min(2),
        ])
        .split(popup_area);


    frame.render_widget(Clear, popup_area);

    let popup_block = Block::bordered()
        .border_style(Style::new()
            .fg(Color::Cyan))
        .border_type(BorderType::Plain)
        .title(Line::from(menu.title.clone())
            .alignment(Alignment::Center)
            .style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightBlue)));

    frame.render_widget(popup_block, popup_area);


    let buttons_chunk = menu_buttons_place_rect(popup_chunks[1], &menu.items);

    for (index, item) in menu.items.iter().enumerate() {
        let style = if index == menu.selected {
            Style::new().fg(Color::DarkGray).bg(Color::Yellow)
        } else {
            Style::new().fg(Color::Gray)
        };


        let text = Span::from(item.name.as_str())
            .style(style);

        frame.render_widget(
            Paragraph::new(text)
                .alignment(Alignment::Center),
            buttons_chunk[index]
        );
    }



}
