use ratatui::style::Color;

pub(crate) struct Menu {
    pub(crate) title: String,
    pub(crate) items: Vec<Item>,
    pub(crate) selected: usize
}

pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) action: Box<dyn Fn()>,
}
