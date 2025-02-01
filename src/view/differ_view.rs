use std::f32;

use egui::{Color32, FontId, RichText, TextFormat, Widget};

use crate::model::TextDifference;

use super::View;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferenceView {
    pub input1: String,
    pub input2: String,
    pub differs: Vec<(String, String)>,
    pub compared: bool,
}

impl DifferenceView {
    pub fn new() -> Self {
        Self {
            input1: String::new(),
            input2: String::new(),
            differs: Vec::new(),
            compared: false,
        }
    }

    fn _create_colored_editor(
        ui: &mut egui::Ui,
        text: &mut String,
        is_original: bool,
        max_width: f32,
        differs: &[(String, String)], // 通过参数传入差异数据
    ) -> egui::Response {
        // let differs = &self.differs;

        let mut layouter = move |ui: &egui::Ui, _: &str, wrap_width: f32| {
            let mut layout_job = egui::text::LayoutJob::default();

            for (word, flag) in differs {
                let color = match flag.as_str() {
                    "-" if is_original => Color32::RED,
                    "+" if !is_original => Color32::GREEN,
                    _ => Color32::TRANSPARENT,
                };

                let should_display = (is_original && flag != "+") || (!is_original && flag != "-");

                if should_display {
                    layout_job.append(
                        word,
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color,
                            ..Default::default()
                        },
                    );
                    layout_job.append(" ", 0.0, TextFormat::default());
                }
            }

            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::TextEdit::multiline(text)
            .font(egui::TextStyle::Monospace)
            .code_editor()
            .desired_rows(20)
            .desired_width(max_width)
            .layouter(&mut layouter)
            .ui(ui)
    }

    fn calculate_diff(&mut self) {
        self.differs = if !self.input1.is_empty() && !self.input2.is_empty() {
            TextDifference::differ(&self.input1, &self.input2)
                .into_iter()
                // 过滤有效差异
                .filter(|(_, flag)| flag == "-" || flag == "+")
                // .filter(|(val, _)| val != "\n")
                .collect()
        } else {
            Vec::new()
        };

        println!("{:?}", self.differs);
    }
}

impl View for DifferenceView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Text Difference");
            ui.separator();
            ui.label(
                RichText::new("Compare two texts and see the differences between them.")
                    .text_style(egui::TextStyle::Small),
            );
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if !self.compared {
                        if ui
                            .link(RichText::new("Compare").text_style(egui::TextStyle::Button))
                            .clicked()
                        {
                            self.calculate_diff();
                            self.compared = true;
                        }
                    } else {
                        if ui
                            .link(RichText::new("Reset").text_style(egui::TextStyle::Button))
                            .clicked()
                        {
                            self.compared = false;
                            self.differs.clear();
                        }
                    }
                });
                ui.add_space(18.0);
            });

            ui.horizontal(|ui| {
                let max_width = ui.available_width();

                let res1 = ui.add(
                    egui::TextEdit::multiline(&mut self.input1)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(20)
                        .desired_width(max_width * 0.46),
                );

                let res2 = ui.add(
                    egui::TextEdit::multiline(&mut self.input2)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(20)
                        .desired_width(max_width * 0.46),
                );

                if res1.changed() || res2.changed() {
                    self.calculate_diff();
                }
            });

            ui.vertical(|ui| {
                ui.separator();
                ui.label(RichText::new("Compare Result :").strong());

                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        let mut layouter = |ui: &egui::Ui, _: &str, wrap_width: f32| {
                            let mut layout_job = egui::text::LayoutJob::default();
                            for (text, flag) in &self.differs {
                                let color = match flag.as_str() {
                                    "-" => Color32::LIGHT_RED,
                                    "+" => Color32::LIGHT_GREEN,
                                    _ => Color32::GRAY,
                                };

                                layout_job.append(
                                    text,
                                    0.0,
                                    TextFormat {
                                        font_id: FontId::monospace(14.0),
                                        background: color,
                                        ..Default::default()
                                    },
                                );

                                // 显式添加换行符
                                layout_job.append(
                                    "\n",
                                    0.0,
                                    TextFormat {
                                        font_id: FontId::monospace(14.0),
                                        ..Default::default()
                                    },
                                );
                            }

                            layout_job.wrap.max_width = wrap_width;
                            ui.fonts(|f| f.layout_job(layout_job))
                        };

                        let mut text = self
                            .differs
                            .iter()
                            .map(|(w, f)| w.to_string())
                            .collect::<Vec<String>>()
                            .join("\n");

                        egui::TextEdit::multiline(&mut text)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_rows(10)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter)
                            .ui(ui);
                    });
            });
        });
    }
}
