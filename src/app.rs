use developer_tools::view::{
    calculator_view::CalcView, datetime_view::DateTimeConverterView, differ_view::DifferenceView,
    fish_view::StockView, formatter_view::FormatterView, game_of_life::GameOfLifeView,
    hash_view::HashView, number_view::NumberBaseConverterView, View, WindowView,
};
use eframe::egui;
use egui::{epaint::color, Color32, Frame, RichText, TextStyle};
// use webbrowser;

#[derive(Debug, Clone, PartialEq)]
enum DeveloperTools {
    Hashing(HashView),
    Calculator(CalcView),
    TextDifference(DifferenceView),
    JsonFormatter(FormatterView),
    SqlFormatter(FormatterView),
    DateTimeConverter(DateTimeConverterView),
    NumberBaseConverter(NumberBaseConverterView),
    GameOfLife(GameOfLifeView),
    // MOFish(StockView),
}

impl Default for DeveloperTools {
    fn default() -> Self {
        Self::Hashing(HashView::new())
    }
}

impl DeveloperTools {
    fn name(&self) -> &str {
        match self {
            DeveloperTools::Hashing(_v) => "Hashing",
            DeveloperTools::Calculator(_v) => "Calculator",
            DeveloperTools::TextDifference(_v) => "Text Difference",
            // DeveloperTools::Base64 => "Base64",
            DeveloperTools::JsonFormatter(_v) => "Json Formatter",
            DeveloperTools::SqlFormatter(_v) => "Sql Formatter",
            DeveloperTools::DateTimeConverter(_v) => "Date Time Converter",
            DeveloperTools::NumberBaseConverter(_v) => "Number Base Converter",
            DeveloperTools::GameOfLife(_v) => "Game of Life",
            // DeveloperTools::MOFish(_v) => "mo fish",
        }
    }

    fn view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        match self {
            DeveloperTools::Hashing(v) => v.render(ctx, ui),
            DeveloperTools::Calculator(v) => v.render(ctx, ui),
            DeveloperTools::JsonFormatter(v) => v.render(ctx, ui),
            DeveloperTools::SqlFormatter(v) => v.render(ctx, ui),
            DeveloperTools::DateTimeConverter(v) => v.render(ctx, ui),
            DeveloperTools::NumberBaseConverter(v) => v.render(ctx, ui),
            DeveloperTools::TextDifference(v) => v.render(ctx, ui),
            DeveloperTools::GameOfLife(v) => v.render(ctx, ui),
            // DeveloperTools::MOFish(v) => {}
        }
    }
}

// 主应用结构

pub struct App {
    selected_tool_index: Option<usize>, // 使用索引代替克隆
    developer_tools: Vec<DeveloperTools>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_tool_index: None,
            developer_tools: vec![
                DeveloperTools::Hashing(HashView::new()),
                DeveloperTools::Calculator(CalcView::default()),
                DeveloperTools::JsonFormatter(FormatterView::new("json")),
                DeveloperTools::SqlFormatter(FormatterView::new("sql")),
                DeveloperTools::DateTimeConverter(DateTimeConverterView::new()),
                DeveloperTools::NumberBaseConverter(NumberBaseConverterView::new()),
                DeveloperTools::TextDifference(DifferenceView::new()),
                DeveloperTools::GameOfLife(GameOfLifeView::default()),
                // DeveloperTools::MOFish(StockView::new()),
            ],
        }
    }

    pub fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(
                RichText::new("Developer Tools")
                    .text_style(TextStyle::Name("logo".into()))
                    .color(Color32::from_rgb(144, 125, 172)),
            );
            ui.separator();

            // 工具列表区域
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for (index, tool) in self.developer_tools.iter().enumerate() {
                    if ui
                        .selectable_label(
                            self.selected_tool_index.map_or(false, |s| s == index),
                            RichText::new(tool.name()).size(12.0),
                        )
                        .clicked()
                    {
                        self.selected_tool_index = Some(index);
                    }
                    ui.add_space(8.0);
                }
            });
        });
    }

    pub fn render_main_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            //top
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                ui.horizontal(|ui| {
                    let github_icon = egui::include_image!("../assets/icons/github.png");

                    let button = egui::Button::image(github_icon).frame(false);
                    if ui.add(button).clicked() {
                        ctx.output_mut(|o| {
                            o.open_url = Some(egui::output::OpenUrl {
                                url: "https://github.com/chrono36/delveloper-tools".to_owned(),
                                new_tab: true,
                            });
                        });
                    }
                    egui::global_theme_preference_buttons(ui);
                });
            });

            ui.add_space(15.0);

            // Main content area
            if let Some(index) = self.selected_tool_index {
                if let Some(tool) = self.developer_tools.get_mut(index) {
                    tool.view(ctx, ui) // 直接操作原实例
                }
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .max_width(172.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_panel(ctx, ui);
        });
    }
}
