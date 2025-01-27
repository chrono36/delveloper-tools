use super::{Tool, ToolLogic, ToolState, ToolUI};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use eframe::egui;

#[derive(Default)]
pub struct HexToolState {
    input: String,
    output: String,
    error: String,
    input_base: NumberBase,
    output_base: NumberBase,
    is_dirty: bool,
}

impl ToolState for HexToolState {
    fn clear(&mut self) {
        self.input.clear();
        self.output.clear();
        self.error.clear();
        self.is_dirty = false;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}

impl ToolLogic for HexTool {
    fn process(&mut self) -> Result<(), String> {
        if !self.validate() {
            return Err("输入不能为空".to_string());
        }
        self.convert();
        if !self.state.error.is_empty() {
            return Err(self.state.error.clone());
        }
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.input.trim().is_empty()
    }
}

impl ToolUI for HexTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.ui(ui);
    }

    fn name(&self) -> &str {
        "Number Base Converter"
    }

    fn description(&self) -> &str {
        "进制转换工具"
    }
}

impl Tool for HexTool {
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

impl ToolState for HexTool {
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

pub struct HexTool {
    state: HexToolState,
}

#[derive(PartialEq, Default, Debug)]
enum NumberBase {
    #[default]
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
    Base64,
}

impl Default for HexTool {
    fn default() -> Self {
        Self {
            state: HexToolState::default(),
        }
    }
}

impl HexTool {
    pub fn new() -> Self {
        Self::default()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Number Base Converter");
            ui.label("Convert numbers between different bases");
            ui.add_space(8.0);

            // Input area with improved layout
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Input:");
                    let text_edit = egui::TextEdit::singleline(&mut self.state.input)
                        .desired_width(ui.available_width() - 150.0)
                        .hint_text("Enter a number...");
                    if ui.add(text_edit).changed() {
                        self.state.is_dirty = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Input base:");
                    let combo = egui::ComboBox::from_id_source("input_base")
                        .selected_text(format!("{:?}", self.state.input_base))
                        .width(150.0);
                    combo.show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.input_base,
                            NumberBase::Binary,
                            "Binary (2)",
                        );
                        ui.selectable_value(
                            &mut self.state.input_base,
                            NumberBase::Octal,
                            "Octal (8)",
                        );
                        ui.selectable_value(
                            &mut self.state.input_base,
                            NumberBase::Decimal,
                            "Decimal (10)",
                        );
                        ui.selectable_value(
                            &mut self.state.input_base,
                            NumberBase::Hexadecimal,
                            "Hexadecimal (16)",
                        );
                        ui.selectable_value(
                            &mut self.state.input_base,
                            NumberBase::Base64,
                            "Base64",
                        );
                    });
                });
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("Convert").clicked() {
                    self.convert();
                }
                if ui.button("Clear").clicked() {
                    self.state.clear();
                }
            });

            ui.add_space(8.0);

            // Output area with improved error handling and layout
            if !self.state.error.is_empty() {
                ui.group(|ui| {
                    ui.colored_label(egui::Color32::RED, &self.state.error);
                });
            } else if !self.state.input.is_empty() {
                ui.group(|ui| {
                    ui.label("Results:");
                    ui.add_space(4.0);

                    // Binary output
                    self.format_output_row(ui, "Binary (2)", self.format_binary());

                    // Octal output
                    self.format_output_row(ui, "Octal (8)", self.format_octal());

                    // Decimal output
                    self.format_output_row(ui, "Decimal (10)", self.format_decimal());

                    // Hexadecimal output
                    self.format_output_row(ui, "Hexadecimal (16)", self.format_hexadecimal());

                    // Base64 output
                    self.format_output_row(ui, "Base64", self.format_base64());
                });
            }
        });
    }

    fn format_output_row(&mut self, ui: &mut egui::Ui, label: &str, content: String) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut output = content.clone();
            ui.add(
                egui::TextEdit::singleline(&mut output).desired_width(ui.available_width() - 50.0),
            );
            if ui.button("📋").clicked() {
                ui.output_mut(|o| o.copied_text = content);
            }
        });
    }

    fn convert(&mut self) {
        self.state.error.clear();
        self.state.output.clear();

        // 首先将输入转换为十进制
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };

        match decimal {
            Ok(num) => {
                self.state.output = match self.state.output_base {
                    NumberBase::Binary => format!("{:b}", num),
                    NumberBase::Octal => format!("{:o}", num),
                    NumberBase::Decimal => num.to_string(),
                    NumberBase::Hexadecimal => format!("{:x}", num),
                    NumberBase::Base64 => {
                        // 移除前导零以避免不必要的填充
                        let bytes: Vec<u8> = num
                            .to_be_bytes()
                            .into_iter()
                            .skip_while(|&x| x == 0)
                            .collect();
                        let bytes = if bytes.is_empty() { vec![0] } else { bytes };
                        base64::engine::general_purpose::STANDARD.encode(bytes)
                    }
                };
            }
            Err(_) => {
                self.state.error = format!(
                    "无效的{}进制数字",
                    match self.state.input_base {
                        NumberBase::Binary => "二",
                        NumberBase::Octal => "八",
                        NumberBase::Decimal => "十",
                        NumberBase::Hexadecimal => "十六",
                        NumberBase::Base64 => "Base64",
                    }
                );
            }
        }
    }

    fn format_binary(&self) -> String {
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };
        match decimal {
            Ok(num) => format!("{:b}", num),
            Err(_) => String::new(),
        }
    }

    fn format_octal(&self) -> String {
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };
        match decimal {
            Ok(num) => format!("{:o}", num),
            Err(_) => String::new(),
        }
    }

    fn format_decimal(&self) -> String {
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };
        match decimal {
            Ok(num) => num.to_string(),
            Err(_) => String::new(),
        }
    }

    fn format_hexadecimal(&self) -> String {
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };
        match decimal {
            Ok(num) => format!("{:x}", num),
            Err(_) => String::new(),
        }
    }

    fn format_base64(&self) -> String {
        let decimal = match self.state.input_base {
            NumberBase::Binary => i64::from_str_radix(self.state.input.trim(), 2),
            NumberBase::Octal => i64::from_str_radix(self.state.input.trim(), 8),
            NumberBase::Decimal => self.state.input.trim().parse::<i64>(),
            NumberBase::Hexadecimal => i64::from_str_radix(self.state.input.trim(), 16),
            NumberBase::Base64 => {
                match base64::engine::general_purpose::STANDARD.decode(self.state.input.trim()) {
                    Ok(bytes) => {
                        if bytes.len() <= 8 {
                            let mut value: i64 = 0;
                            for byte in bytes {
                                value = (value << 8) | (byte as i64);
                            }
                            Ok(value)
                        } else {
                            Err(i64::from_str_radix("invalid", 10).unwrap_err())
                        }
                    }
                    Err(_) => Err(i64::from_str_radix("invalid", 10).unwrap_err()),
                }
            }
        };
        match decimal {
            Ok(num) => {
                let bytes = num.to_be_bytes();
                base64::engine::general_purpose::STANDARD.encode(bytes)
            }
            Err(_) => String::new(),
        }
    }
}
