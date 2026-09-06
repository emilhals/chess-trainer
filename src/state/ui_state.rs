#[derive(Clone, Copy)]
pub enum Views {
    Home,
    Trainer,
}

pub struct UIState {
    pub current_view: Views,
    pub previous_view: Views,
    pub menu_cursor: u8,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            current_view: Views::Trainer,
            previous_view: Views::Home,
            menu_cursor: 0,
        }
    }
}

impl UIState {
    pub fn go_to_homeview(&mut self) {
        self.current_view = Views::Home;
        self.menu_cursor = 0;
    }

    pub fn go_to_previous_view(&mut self) {
        self.current_view = self.previous_view;
    }

    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }

    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }
}
