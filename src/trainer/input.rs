use crate::trainer::Trainer;

#[allow(dead_code)]
impl Trainer {
    pub fn go_up(&mut self) {
        self.ui.cursor_up();
    }

    pub fn go_down(&mut self) {
        self.ui.cursor_down();
    }

    pub fn go_left(&mut self) {
        self.ui.cursor_left();
    }

    pub fn go_right(&mut self) {
        self.ui.cursor_right();
    }

    pub fn process_cell_click(&mut self) {
        // Tracks whether move was correct.
        self.handle_cell_click();
    }
}
