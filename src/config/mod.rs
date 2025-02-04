use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppStyle {
    pub background: Color32,
    pub foreground: Color32,
    pub circle_focus: Color32,
    pub rounds: Color32,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            background: Color32::from_hex("#2f384b").unwrap(),
            foreground: Color32::from_hex("#f0ead6").unwrap(),
            circle_focus: Color32::from_hex("#f25a48").unwrap(),
            rounds: Color32::from_hex("#fceea7").unwrap(),
        }
    }
}
