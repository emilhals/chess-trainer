#[derive(Clone, Copy, PartialEq)]
pub enum MenuFocus {
    ModeSelector,
    Openings,
}

pub struct MenuState {
    pub focus: MenuFocus,
    pub selected_opening_index: usize,
}

impl MenuFocus {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::ModeSelector => Self::Openings,
            Self::Openings => Self::ModeSelector,
        };
    }

    pub fn keybinds(&self) -> &[(&str, &str)] {
        match self {
            Self::ModeSelector => &[("← →", "Change mode"), ("↓", "Openings")],
            Self::Openings => &[("Enter", "Start"), ("← →", "Change opening"), ("↑", "Mode")],
        }
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            focus: MenuFocus::ModeSelector,
            selected_opening_index: 0,
        }
    }
}

impl MenuState {
    pub fn select_next_opening(&mut self, num_openings: usize) {
        if num_openings == 0 {
            return;
        }

        self.selected_opening_index = (self.selected_opening_index + 1) % num_openings;
    }

    pub fn select_previous_opening(&mut self, num_openings: usize) {
        if num_openings == 0 {
            return;
        }

        self.selected_opening_index =
            (self.selected_opening_index + num_openings - 1) % num_openings;
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}
