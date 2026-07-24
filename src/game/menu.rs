pub struct Menu {
    pub(crate) title: String,
    pub(crate) items: Vec<Item>,
    pub(crate) selected: usize
}

pub struct Item {
    pub(crate) name: String,
    pub(crate) action: MenuAction,
}

pub enum MenuAction{
    Resume,
    Restart,
    Quit,
}

impl Menu {
    pub fn new(title: impl Into<String>, items: Vec<Item>) -> Self {
        Self {
            title: title.into(),
            items,
            selected: 0,
        }
    }
    pub fn default(name: impl Into<String>) -> Self {
        Self::new(name.into(), vec![
            Item::restart(),
            Item::quit(),
            Item::resume()
        ])
    }
    pub(crate) fn execute(&self) -> &MenuAction {
        self.items.get(self.selected).map(|item| &item.action).unwrap()
    }

}
impl Item {
    pub fn new(name: impl Into<String>, action: MenuAction) -> Self {
        Self { name: name.into(), action }
    }
    pub fn resume() -> Self{
        Self::new("Resume", MenuAction::Resume)
    }
    pub fn quit() -> Self{
        Self::new("Quit", MenuAction::Quit)
    }
    pub fn restart() -> Self{
        Self::new("Restart", MenuAction::Restart)
    }
}
