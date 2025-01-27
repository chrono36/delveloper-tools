use crate::tools::ToolState;
use eframe::egui;
use sqlformat::{FormatOptions, Indent, QueryParams};

pub struct SqlTool {
    input_text: String,
    output_text: String,
    error_msg: String,
    is_dirty: bool,
    indent_size: usize,
}

impl Default for SqlTool {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            error_msg: String::new(),
            is_dirty: false,
            indent_size: 2,
        }
    }
}

impl SqlTool {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // ui.label("缩进大小:");
                ui.add(egui::Slider::new(&mut self.indent_size, 1..=8));
                if ui.button("format").clicked() && !self.input_text.trim().is_empty() {
                    self.format_sql();
                }
                if ui.button("Clear").clicked() {
                    self.clear();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("input your SQL:");
                    let text_edit = egui::TextEdit::multiline(&mut self.input_text)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    if ui.add(text_edit).changed() {
                        self.is_dirty = true;
                    }
                });

                ui.vertical(|ui| {
                    ui.label("format result:");
                    let mut output = self.output_text.clone();
                    let text_edit = egui::TextEdit::multiline(&mut output)
                        .desired_width(ui.available_width() * 0.45)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    ui.add(text_edit);
                    if !self.output_text.is_empty()
                        && ui.button("📋").on_hover_text("复制到剪贴板").clicked()
                    {
                        ui.output_mut(|o| o.copied_text = self.output_text.clone());
                    }
                });
            });

            if !self.error_msg.is_empty() {
                ui.label(egui::RichText::new(&self.error_msg).color(egui::Color32::RED));
            }
        });
    }

    fn format_sql(&mut self) {
        let options = FormatOptions {
            indent: Indent::Spaces(self.indent_size.try_into().unwrap()),
            uppercase: Some(true),
            lines_between_queries: 2,
            ignore_case_convert: None,
        };

        let result = sqlformat::format(&self.input_text, &QueryParams::None, &options);
        self.output_text = result;
        self.error_msg.clear();
        self.is_dirty = true;
    }
}

impl super::ToolState for SqlTool {
    fn clear(&mut self) {
        self.input_text.clear();
        self.output_text.clear();
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

impl super::ToolLogic for SqlTool {
    fn process(&mut self) -> Result<(), String> {
        if !self.validate() {
            return Err("SQL 语句不能为空".to_string());
        }
        self.format_sql();
        if !self.error_msg.is_empty() {
            return Err(self.error_msg.clone());
        }
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.input_text.trim().is_empty()
    }
}

impl super::ToolUI for SqlTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.ui(ui);
    }

    fn name(&self) -> &str {
        "SQL Formatter"
    }

    fn description(&self) -> &str {
        "提供 SQL 语句的格式化功能，支持多种 SQL 语法"
    }
}

impl super::Tool for SqlTool {
    fn as_state(&self) -> &dyn super::ToolState {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn super::ToolState {
        self
    }

    fn as_logic(&self) -> &dyn super::ToolLogic {
        self
    }

    fn as_logic_mut(&mut self) -> &mut dyn super::ToolLogic {
        self
    }

    fn as_ui(&self) -> &dyn super::ToolUI {
        self
    }

    fn as_ui_mut(&mut self) -> &mut dyn super::ToolUI {
        self
    }
}
