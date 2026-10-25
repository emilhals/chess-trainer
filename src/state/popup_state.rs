#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Popups {
    Settings,
    Confirmation(ConfirmationAction),
}

#[derive(Clone, Copy, PartialEq)]
pub enum SettingsTabs {
    General,
    Appearance,
    Trainer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConfirmationAction {
    ResetProgress,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Button {
    Confirm,
    Cancel,
}

pub struct PopupState {
    pub active_popup: Option<Popups>,
    pub selected_tab: SettingsTabs,
    pub selected_option: usize,
    pub selected_button: Button,
}

impl SettingsTabs {
    pub fn shortcuts(&self, selected_option: usize) -> &[(&str, &str)] {
        match self {
            Self::General => &[],
            Self::Appearance => match selected_option {
                0 => &[("Space", "Toggle")],
                1 => &[("← →", "Adjust")],
                _ => &[],
            },
            Self::Trainer => match selected_option {
                0..=2 => &[("← →", "Adjust")],
                3 => &[("Space", "Toggle")],
                4 => &[("Enter", "Select")],
                _ => &[],
            },
        }
    }
}

impl ConfirmationAction {
    pub fn title(&self) -> &str {
        match self {
            Self::ResetProgress => "Reset progress?",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::ResetProgress => "This will permanently delete all training progress.",
        }
    }
}

impl PopupState {
    pub fn open_settings(&mut self) {
        self.active_popup = Some(Popups::Settings);
    }

    pub fn open_confirmation(&mut self, action: ConfirmationAction) {
        self.active_popup = Some(Popups::Confirmation(action));
        self.selected_button = Button::Cancel;
    }

    pub fn close(&mut self) {
        self.active_popup = None;
    }

    pub fn toggle_button(&mut self) {
        match self.selected_button {
            Button::Confirm => self.selected_button = Button::Cancel,
            Button::Cancel => self.selected_button = Button::Confirm,
        }
    }
}
