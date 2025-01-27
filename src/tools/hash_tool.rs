use super::{Tool, ToolLogic, ToolState, ToolUI};
use eframe::egui;
use md5::{Digest, Md5};
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha224, Sha256, Sha384, Sha512};
use sha3::{Digest as Sha3Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

#[derive(Default)]
pub struct HashToolState {
    input: String,
    output: Vec<(String, String)>, // (algorithm, hash)
    selected_algorithm: String,
    is_dirty: bool,
}

impl ToolState for HashToolState {
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

pub struct HashTool {
    state: HashToolState,
}

impl Default for HashTool {
    fn default() -> Self {
        Self {
            state: HashToolState::default(),
        }
    }
}

impl ToolLogic for HashTool {
    fn process(&mut self) -> Result<(), String> {
        if self.state.input.is_empty() {
            return Err("输入不能为空".to_string());
        }

        self.state.output.clear();
        let input = self.state.input.as_bytes();

        // MD5
        let mut hasher = Md5::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("MD5".to_string(), format!("{:x}", result)));

        // SHA1
        let mut hasher = Sha1::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA1".to_string(), format!("{:x}", result)));

        // SHA256
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA256".to_string(), format!("{:x}", result)));

        // SHA224
        let mut hasher = Sha224::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA224".to_string(), format!("{:x}", result)));

        // SHA512
        let mut hasher = Sha512::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA512".to_string(), format!("{:x}", result)));

        // SHA384
        let mut hasher = Sha384::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA384".to_string(), format!("{:x}", result)));

        // SHA3
        let mut hasher = Sha3_224::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("SHA3".to_string(), format!("{:x}", result)));

        // RIPEMD160
        let mut hasher = Ripemd160::new();
        hasher.update(input);
        let result = hasher.finalize();
        self.state
            .output
            .push(("RIPEMD160".to_string(), format!("{:x}", result)));

        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.input.is_empty()
    }
}

impl ToolUI for HashTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Hash text");
            ui.label("Hash a text string using the function you need: MD5, SHA1, SHA256, SHA224, SHA512, SHA384, SHA3 or RIPEMD160");
            ui.add_space(8.0);

            ui.label("Your text to hash:");
            if ui.text_edit_multiline(&mut self.state.input).changed() {
                self.state.is_dirty = true;
            }

            ui.add_space(8.0);
            ui.label("Digest encoding");
            ui.label("Hexadecimal (base 16)");

            ui.horizontal(|ui| {
                if ui.button("Calculate").clicked() && self.validate() {
                    if let Err(e) = self.process() {
                        self.state.output = vec![("Error".to_string(), e)];
                    }
                }
                if ui.button("Clear").clicked() {
                    self.state.clear();
                }
            });

            ui.add_space(8.0);
            for (algo, hash) in &self.state.output {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(algo).on_hover_text("Hash algorithm");
                        ui.add_space(4.0);
                        let mut hash_clone = hash.clone();
                        ui.add(egui::TextEdit::singleline(&mut hash_clone)
                            .desired_width(ui.available_width() - 60.0)
                            .font(egui::TextStyle::Monospace));
                        if ui.button("📋").on_hover_text("Copy to clipboard").clicked() {
                            ui.output_mut(|o| o.copied_text = hash.clone());
                        }
                    });
                });
            }
        });
    }

    fn name(&self) -> &str {
        "Hash Text"
    }

    fn description(&self) -> &str {
        "MD5, SHA1, SHA256, SHA512等文本加密"
    }
}

impl Tool for HashTool {
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

impl ToolState for HashTool {
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
