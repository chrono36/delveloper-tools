use super::{Tool, ToolLogic, ToolState, ToolUI};
use eframe::egui::{self, RichText};
use serde_json::{from_str, to_string_pretty, Value};

pub struct JsonTool {
    input: String,
    output: String,
    error_msg: String,
    indent: usize,
    is_dirty: bool,
}

impl Default for JsonTool {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            error_msg: String::new(),
            indent: 2,
            is_dirty: false,
        }
    }
}

impl JsonTool {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("JSON Formatter");
            ui.label("Format, validate and beautify your JSON data");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Indent size:");
                ui.add(egui::Slider::new(&mut self.indent, 1..=8));
                if ui.button("Format").clicked() && !self.input.trim().is_empty() {
                    self.format_json();
                }
                if ui.button("Clear").clicked() {
                    self.clear();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Input your JSON:");
                    let text_edit = egui::TextEdit::multiline(&mut self.input)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    if ui.add(text_edit).changed() {
                        self.is_dirty = true;
                    }
                });

                ui.vertical(|ui| {
                    ui.label("Format result:");
                    let mut output = self.output.clone();
                    let text_edit = egui::TextEdit::multiline(&mut output)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    ui.add(text_edit);
                    if !self.output.is_empty()
                        && ui.button("📋").on_hover_text("复制到剪贴板").clicked()
                    {
                        ui.output_mut(|o| o.copied_text = self.output.clone());
                    }
                });
            });

            if !self.error_msg.is_empty() {
                ui.add_space(8.0);
                ui.label(RichText::new(&self.error_msg).color(egui::Color32::RED));
            }
        });
    }

    fn format_json(&mut self) {
        self.error_msg.clear();
        match from_str::<Value>(&self.input) {
            Ok(json) => match to_string_pretty(&json) {
                Ok(formatted) => {
                    self.output = formatted;
                    self.is_dirty = false;
                }
                Err(e) => self.error_msg = format!("格式化错误: {}", e),
            },
            Err(e) => self.error_msg = format!("JSON 解析错误: {}", e),
        }
    }
}

impl Tool for JsonTool {
    fn as_state(&self) -> &dyn ToolState {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn ToolState {
        self
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

impl ToolState for JsonTool {
    fn clear(&mut self) {
        self.input.clear();
        self.output.clear();
        self.error_msg.clear();
        self.is_dirty = false;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}

impl ToolLogic for JsonTool {
    fn process(&mut self) -> Result<(), String> {
        if !self.validate() {
            return Err("JSON 文本不能为空".to_string());
        }
        self.format_json();
        if !self.error_msg.is_empty() {
            return Err(self.error_msg.clone());
        }
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.input.trim().is_empty()
    }
}

impl ToolUI for JsonTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.ui(ui);
    }

    fn name(&self) -> &str {
        "JSON Format"
    }

    fn description(&self) -> &str {
        "JSON 格式化工具"
    }
}
