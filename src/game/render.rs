use super::snake::{Snake, SnakeDirection};
use ratatui::prelude::{Rect, Buffer, Widget};

impl Widget for &Snake {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (symbol, (x,y)) in self.iter_parts() {
            if x<0 || y< 0{
                continue;
            }

            let screen_x = area.x + x as u16;
            let screen_y = area.y + y as u16;

            if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
                cell.set_char(symbol);
            }
        }
    }
}
