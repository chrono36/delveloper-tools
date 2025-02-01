use super::WindowView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockView {}

impl StockView {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowView for StockView {
    fn name(&self) -> &'static str {
        "Stocks list"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.label("stocks list ..."));
        });
    }
}
