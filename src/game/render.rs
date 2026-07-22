//! # Модуль для реализации trait'а [`Widget`] для модулей от модуля `game`

use super::menu::Menu;
use super::snake::Snake;
use ratatui::prelude::Widget;
use ratatui::style::Color;

impl Widget for &Snake {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        for (symbol, (x,y)) in self.iter_parts() {
            if x<0 || y< 0{
                continue;
            }

            let symbol = self.symbols[&symbol];

            let screen_x = area.x + x as u16;
            let screen_y = area.y + y as u16;

            if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
                cell.set_char(symbol);
            }
        }
    }
}
