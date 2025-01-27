use super::{Tool, ToolLogic, ToolState, ToolUI};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use eframe::egui;

#[derive(Default)]
pub struct Base64ToolState {
    input: String,
    output: String,
    is_encode: bool,
    is_dirty: bool,
}

impl ToolState for Base64ToolState {
    fn clear(&mut self) {
        self.input.clear();
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

pub struct Base64Tool {
    state: Base64ToolState,
}

impl Default for Base64Tool {
    fn default() -> Self {
        Self {
            state: Base64ToolState::default(),
        }
    }
}

impl ToolLogic for Base64Tool {
    fn process(&mut self) -> Result<(), String> {
        if self.state.input.is_empty() {
            return Err("输入不能为空".to_string());
        }

        self.state.output = if self.state.is_encode {
            STANDARD.encode(self.state.input.as_bytes())
        } else {
            match STANDARD.decode(self.state.input.as_bytes()) {
                Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
                Err(e) => return Err(format!("解码失败: {}", e)),
            }
        };

        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.input.is_empty()
    }
}

impl ToolUI for Base64Tool {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Base64 encoder/decoder");

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.state.is_encode, true, "Encode");
                ui.radio_value(&mut self.state.is_encode, false, "Decode");
            });

            // ui.label(if self.state.is_encode { "要编码的文本:" } else { "要解码的文本:" });
            ui.label("input your text here:");
            if ui.text_edit_multiline(&mut self.state.input).changed() {
                self.state.is_dirty = true;
            }

            ui.horizontal(|ui| {
                if ui.button("Done").clicked() && self.validate() {
                    if let Err(e) = self.process() {
                        self.state.output = format!("error: {}", e);
                    }
                }
                if ui.button("Clear").clicked() {
                    self.state.clear();
                }
            });

            if !self.state.output.is_empty() {
                ui.add_space(8.0);
                // ui.label(if self.state.is_encode { "编码结果:" } else { "解码结果:" });
                let mut output = self.state.output.clone();
                ui.add(
                    egui::TextEdit::multiline(&mut output)
                        .desired_width(ui.available_width())
                        .font(egui::TextStyle::Monospace),
                );
                if ui.button("📋").on_hover_text("复制到剪贴板").clicked() {
                    ui.output_mut(|o| o.copied_text = self.state.output.clone());
                }
            }
        });
    }

    fn name(&self) -> &str {
        "Base64"
    }

    fn description(&self) -> &str {
        "Base64 Encoder/Decoder"
    }
}

impl Tool for Base64Tool {
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

impl ToolState for Base64Tool {
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
