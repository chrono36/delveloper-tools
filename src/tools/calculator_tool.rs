use super::{Tool, ToolLogic, ToolState, ToolUI};
// use crate::tools::calculator::Expr;
use eframe::egui;

#[derive(Default)]
pub struct CalculatorToolState {
    input: String,
    history: Vec<(String, String)>,
    error: String,
    is_dirty: bool,
}

impl ToolState for CalculatorToolState {
    fn clear(&mut self) {
        self.input.clear();
        self.history.clear();
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

pub struct CalculatorTool {
    state: CalculatorToolState,
}

impl Default for CalculatorTool {
    fn default() -> Self {
        Self {
            state: CalculatorToolState::default(),
        }
    }
}

impl CalculatorTool {
    pub fn new() -> Self {
        Self::default()
    }

    // fn calculate(&mut self) {
    //     let input = self.state.input.trim();
    //     if input.is_empty() {
    //         return;
    //     }

    //     let mut expr = Expr::new(input);
    //     match expr.eval() {
    //         Ok(result) => {
    //             self.state
    //                 .history
    //                 .push((input.to_string(), result.to_string()));
    //             self.state.input.clear();
    //             self.state.error.clear();
    //         }
    //         Err(e) => {
    //             self.state.error = e.to_string();
    //         }
    //     }
    //     self.state.is_dirty = true;
    // }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Calculator");
            ui.label("Enter a mathematical expression and press Enter to calculate");
            ui.add_space(8.0);

            // Input area
            let text_edit = egui::TextEdit::singleline(&mut self.state.input)
                .desired_width(ui.available_width() - 20.0)
                .hint_text("Example: 1 + 2 * 3");
            let response = ui.add(text_edit);

            // Handle Enter key press
            // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            //     self.calculate();
            // }

            // Error display
            if !self.state.error.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.state.error);
            }

            ui.add_space(8.0);

            // History area
            if !self.state.history.is_empty() {
                ui.group(|ui| {
                    ui.label("History:");
                    ui.add_space(4.0);

                    for (expr, result) in self.state.history.iter().rev() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{} = {}", expr, result));
                            if ui.button("📋").clicked() {
                                ui.output_mut(|o| o.copied_text = result.clone());
                            }
                        });
                    }
                });
            }
        });
    }
}

impl Tool for CalculatorTool {
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

impl ToolLogic for CalculatorTool {
    fn process(&mut self) -> Result<(), String> {
        // self.calculate();
        if !self.state.error.is_empty() {
            return Err(self.state.error.clone());
        }
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.input.trim().is_empty()
    }
}

impl ToolUI for CalculatorTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.ui(ui);
    }

    fn name(&self) -> &str {
        "Calculator"
    }

    fn description(&self) -> &str {
        "数学表达式计算器"
    }
}

impl ToolState for CalculatorTool {
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
