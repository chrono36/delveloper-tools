use egui::{output, Layout, RichText};

use crate::model::Expr;

use super::View;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CalcView {
    pub input: String,
    pub output: Vec<String>,
    pub err_msg: String,
}

impl CalcView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: Vec::new(),
            err_msg: String::new(),
        }
    }

    fn calculate(&mut self) {
        let input = self.input.trim();
        if input.is_empty() {
            return;
        }

        let mut expr = Expr::new(input);
        match expr.eval() {
            Ok(result) => {
                self.output.push(format!("{} = {}", self.input, result));
            }
            Err(e) => {
                self.err_msg = e.to_string();
            }
        }
        self.input = String::new();
    }
}

impl View for CalcView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Calculator");
            ui.separator();
            ui.label(
                RichText::new("Support addition, subtraction, multiplication, division and power")
                    .text_style(egui::TextStyle::Small),
            );
            ui.add_space(15.0);
            let text_edit = egui::TextEdit::multiline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(ui.available_width())
                .hint_text("Enter Mathematical expressions to calacutor ...");
            let response = ui.add(text_edit);

            // if response.changed() {
            //     self.err_msg = String::new();
            // }

            // Handle Enter key press
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.calculate();
            }

            ui.add_space(10.0);
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Calcuator").clicked() {
                    self.calculate();
                }
            });

            if !self.err_msg.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.err_msg);
            }

            self.output.iter().for_each(|res| {
                ui.label(res);
            });
        });
    }
}
