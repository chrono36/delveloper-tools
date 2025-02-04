pub mod calculator_view;
pub mod datetime_view;
pub mod differ_view;
pub mod fish_view;
pub mod formatter_view;
pub mod game_of_life;
pub mod hash_view;
pub mod number_view;

pub trait View {
    fn render(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("is default view");
    }
}

pub trait WindowView {
    /// Is the demo enabled for this integration?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
