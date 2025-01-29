use developer_tools::view::{hash_view::HashView, DefaultView, View};
use eframe::egui;
use egui::{RichText, TextStyle};
// use webbrowser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeveloperTools {
    Hashing(HashView),
    Calculator(DefaultView),
    // TextDifference,
    // Base64,
    // JsonFormatter,
    // SqlFormatter,
    // DateTimeConverter,
    // NumberBaseConverter,
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
            // DeveloperTools::TextDifference => "Text Difference",
            // DeveloperTools::Base64 => "Base64",
            // DeveloperTools::JsonFormatter => "Json Formatter",
            // DeveloperTools::SqlFormatter => "Sql Formatter",
            // DeveloperTools::DateTimeConverter => "Date Time Converter",
            // DeveloperTools::NumberBaseConverter => "Number Base Converter",
        }
    }

    fn view(&mut self, ui: &mut egui::Ui) {
        match self {
            DeveloperTools::Hashing(v) => v.render(ui),
            DeveloperTools::Calculator(v) => v.render(ui),
        }
    }

    // fn view(&self) -> Box<dyn View> {
    //     match self {
    //         DeveloperTools::Hashing(v) => Box::new(v),
    //         DeveloperTools::Calculator => Box::new(developer_tools::view::DefaultView::default()),
    //         // DeveloperTools::TextDifference => {
    //         //     Box::new(developer_tools::view::DefaultView::default())
    //         // }
    //         // DeveloperTools::Base64 => Box::new(developer_tools::view::DefaultView::default()),
    //         // DeveloperTools::JsonFormatter => {
    //         //     Box::new(developer_tools::view::DefaultView::default())
    //         // }
    //         // DeveloperTools::SqlFormatter => Box::new(developer_tools::view::DefaultView::default()),
    //         // DeveloperTools::DateTimeConverter => {
    //         //     Box::new(developer_tools::view::DefaultView::default())
    //         // }
    //         // DeveloperTools::NumberBaseConverter => {
    //         //     Box::new(developer_tools::view::DefaultView::default())
    //         // }
    //     }
    // }
}

// 主应用结构

pub struct App {
    selected_tool_index: Option<usize>, // 使用索引代替克隆
    developer_tools: Vec<DeveloperTools>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        println!("init ...");
        Self {
            selected_tool_index: None,
            developer_tools: vec![
                DeveloperTools::Hashing(HashView::new()),
                DeveloperTools::Calculator(DefaultView::default()),
            ],
        }
    }

    pub fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Developer Tools").text_style(TextStyle::Name("logo".into())));
            ui.separator();

            // 工具列表区域
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for (index, tool) in self.developer_tools.iter().enumerate() {
                    if ui
                        .selectable_label(
                            self.selected_tool_index.map_or(false, |s| s == index),
                            tool.name(),
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

    pub fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            //top
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                ui.horizontal(|ui| {
                    // egui::widgets::global_theme_preference_switch(ui);
                    if ui.button("GitHub").clicked() {
                        // if let Err(e) = webbrowser::open("https://github.com/chronosp/developer-tools") {
                        //     eprintln!("Failed to open GitHub: {}", e);
                        // }
                    };
                    egui::global_theme_preference_buttons(ui);
                });
            });

            ui.add_space(15.0);

            // Main content area
            if let Some(index) = self.selected_tool_index {
                if let Some(tool) = self.developer_tools.get_mut(index) {
                    tool.view(ui); // 直接操作原实例
                }
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            self.render_sidebar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_panel(ui);
        });
    }
}
