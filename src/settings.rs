use crate::skin::Skins;

pub struct Settings {
    pub selected_skin: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            selected_skin: "chessdotcom".to_string(),
        }
    }
}

impl Settings {
    pub fn cycle_skin(&mut self, skins: Skins, forward: bool) {
        let names = skins.names();

        if names.is_empty() {
            return;
        }

        if let Some(current_index) = names.iter().position(|&skin| skin == self.selected_skin) {
            let next_index;
            if forward {
                next_index = (current_index + 1) % names.len();
            } else {
                next_index = (current_index - 1) % names.len();
            }

            let Some(next_skin) = names.get(next_index) else {
                return;
            };
            self.selected_skin = next_skin.to_string();
        }
    }
}
