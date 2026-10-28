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
    pub fn num_options(&self) -> usize {
        match self {
            Self::General => 0,
            Self::Appearance => 3,
            Self::Trainer => 5,
        }
    }

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

impl Default for PopupState {
    fn default() -> Self {
        Self {
            active_popup: None,
            selected_tab: SettingsTabs::General,
            selected_option: 0,
            selected_button: Button::Cancel,
        }
    }
}

impl PopupState {
    pub fn cycle_tabs(&mut self) {
        match self.selected_tab {
            SettingsTabs::General => {
                self.selected_tab = SettingsTabs::Appearance;
            }
            SettingsTabs::Appearance => {
                self.selected_tab = SettingsTabs::Trainer;
            }
            SettingsTabs::Trainer => {
                self.selected_tab = SettingsTabs::General;
            }
        }

        self.selected_option = 0;
    }

    pub fn cycle_options(&mut self, num_options: usize, forward: bool) {
        if num_options == 0 {
            return;
        }

        let current_option = self.selected_option;

        if forward {
            self.selected_option = (current_option + 1) % num_options;
        } else {
            self.selected_option = (current_option + num_options - 1) % num_options;
        }
    }

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
