use super::{Tool, ToolLogic, ToolState, ToolUI};
use eframe::egui;
use similar::{ChangeTag, TextDiff};

#[derive(Default)]
pub struct DiffToolState {
    text1: String,
    text2: String,
    output: String,
    is_dirty: bool,
}

impl ToolState for DiffToolState {
    fn clear(&mut self) {
        self.text1.clear();
        self.text2.clear();
        self.output.clear();
        self.is_dirty = false;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}

pub struct DiffTool {
    state: DiffToolState,
}

impl Default for DiffTool {
    fn default() -> Self {
        Self {
            state: DiffToolState::default(),
        }
    }
}

impl ToolLogic for DiffTool {
    fn process(&mut self) -> Result<(), String> {
        if self.state.text1.is_empty() || self.state.text2.is_empty() {
            return Err("两段文本都不能为空".to_string());
        }

        let diff = TextDiff::from_lines(&self.state.text1, &self.state.text2);
        let mut output = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            output.push_str(&format!("{}{}", sign, change));
        }

        self.state.output = output;
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.text1.is_empty() && !self.state.text2.is_empty()
    }
}

impl ToolUI for DiffTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Text Differ");
            ui.label("Compare two texts and see the differences between them.");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Original Text");
                    let text_edit = egui::TextEdit::multiline(&mut self.state.text1)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    if ui.add(text_edit).changed() {
                        self.state.is_dirty = true;
                    }
                });
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    ui.label("Modified Text");
                    let text_edit = egui::TextEdit::multiline(&mut self.state.text2)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    if ui.add(text_edit).changed() {
                        self.state.is_dirty = true;
                    }
                });
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("Compare").clicked() && self.validate() {
                    if let Err(e) = self.process() {
                        self.state.output = format!("Error: {}", e);
                    }
                }
                if ui.button("Clear").clicked() {
                    self.state.clear();
                }
            });

            if !self.state.output.is_empty() {
                ui.add_space(8.0);
                ui.label("Differences:");
                ui.add_space(4.0);
                let mut output = self.state.output.clone();
                let text_edit = egui::TextEdit::multiline(&mut output)
                    .desired_width(ui.available_width())
                    .font(egui::TextStyle::Monospace)
                    .text_color(ui.style().visuals.text_color());
                let mut layouter = move |ui: &egui::Ui, text: &str, _wrap_width: f32| {
                    let mut layout_job = egui::text::LayoutJob::default();
                    for line in text.lines() {
                        let (color, background) = match line.chars().next() {
                            Some('-') => (
                                egui::Color32::from_rgb(255, 100, 100),
                                egui::Color32::from_rgba_premultiplied(255, 100, 100, 38),
                            ),
                            Some('+') => (
                                egui::Color32::from_rgb(100, 255, 100),
                                egui::Color32::from_rgba_premultiplied(100, 255, 100, 38),
                            ),
                            _ => (ui.style().visuals.text_color(), egui::Color32::TRANSPARENT),
                        };
                        let format = egui::TextFormat {
                            color,
                            background,
                            font_id: egui::TextStyle::Monospace.resolve(ui.style()),
                            ..Default::default()
                        };
                        layout_job.append(line, 0.0, format);
                        layout_job.append("\n", 0.0, Default::default());
                    }
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                let text_edit = text_edit.layouter(&mut layouter);
                ui.add(text_edit);
                if ui.button("📋").on_hover_text("Copy to clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = self.state.output.clone());
                }
            }
        });
    }

    fn name(&self) -> &str {
        "Text Differ"
    }

    fn description(&self) -> &str {
        "文本差异比较"
    }
}

impl Tool for DiffTool {
    fn as_state(&self) -> &dyn ToolState {
        &self.state
    }

    fn as_state_mut(&mut self) -> &mut dyn ToolState {
        &mut self.state
    }

    fn as_logic(&self) -> &dyn ToolLogic {
        self
    }

    fn as_logic_mut(&mut self) -> &mut dyn ToolLogic {
        self
    }

    fn as_ui(&self) -> &dyn ToolUI {
        self
    }

    fn as_ui_mut(&mut self) -> &mut dyn ToolUI {
        self
    }
}

impl ToolState for DiffTool {
    fn clear(&mut self) {
        self.state.clear();
    }

    fn is_dirty(&self) -> bool {
        self.state.is_dirty()
    }

    fn mark_clean(&mut self) {
        self.state.mark_clean();
    }
}
