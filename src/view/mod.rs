pub mod calculator_view;
pub mod datetime_view;
pub mod formatter_view;
pub mod hash_view;

pub trait View {
    fn render(&mut self, ui: &mut egui::Ui);
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.label("is default view");
    }
}
