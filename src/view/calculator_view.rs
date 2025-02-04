use std::result;

use egui::{output, Layout, RichText};
use egui_extras::{Column, TableBuilder};

use crate::model::Expr;

use super::View;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CalcView {
    pub input: String,
    pub output: Vec<String>,
    pub result: Vec<(String, i32)>,
    pub err_msg: String,
    pub reversed: bool,
}

impl CalcView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: Vec::new(),
            result: Vec::new(),
            reversed: false,
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
                // self.output.push(format!("{} = {}", self.input, result));
                self.result.push((input.to_string(), result));
            }
            Err(e) => {
                self.err_msg = e.to_string();
            }
        }
        self.input = String::new();
    }

    fn render_result(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.separator();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Number");
                        },
                        |ui| {
                            self.reversed ^=
                                ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });

                header.col(|ui| {
                    ui.strong("Expr");
                });
                header.col(|ui| {
                    ui.strong("Result");
                });
            })
            .body(|mut body| {
                for row_index in 0..self.result.len() {
                    let row_index = if self.reversed {
                        self.result.len() - 1 - row_index
                    } else {
                        row_index
                    };

                    body.row(18.0, |mut row| {
                        // row.set_selected(self.selection.contains(&row_index));
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        let item = self.result.get(row_index).unwrap();
                        row.col(|ui| {
                            ui.label(item.0.clone());
                        });
                        row.col(|ui| {
                            ui.label(item.1.to_string());
                        });
                    });
                }
            });
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
            ui.add_space(16.0);

            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add_space(10.0);
                if ui.link(RichText::new("Done").size(12.0)).clicked() {
                    self.calculate();
                }
            });
            ui.add_space(3.0);
            let text_edit = egui::TextEdit::multiline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(ui.available_width() - 20.0)
                .hint_text("Enter Mathematical expressions to calacutor ...");
            let _response = ui.add(text_edit);

            // if response.changed() {
            //     self.err_msg = String::new();
            // }

            // Handle Enter key press
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.calculate();
            }

            if !self.err_msg.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.err_msg);
            }

            // 计算结果放入表格
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_result(ui);
            });
        });
    }
}
