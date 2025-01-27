use developer_tools::tools::{
    base64_tool::Base64Tool, datetime_tool::DateTimeTool, diff_tool::DiffTool, hash_tool::HashTool,
    hex_tool::HexTool, json_tool::JsonTool, sql_tool::SqlTool, Tool,
};
use eframe::egui;
use egui::{RichText, TextStyle};
use std::collections::HashMap;
// use webbrowser;

// 主应用结构
#[derive(Default)]
pub struct DeveloperTools {
    selected_tool: Option<String>,
    search_text: String,
    tools: HashMap<String, Box<dyn Tool>>,
    dark_mode: bool,
}

impl DeveloperTools {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut tools = HashMap::new();
        tools.insert(
            "hash".to_string(),
            Box::new(HashTool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "base64".to_string(),
            Box::new(Base64Tool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "diff".to_string(),
            Box::new(DiffTool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "Json format".to_string(),
            Box::new(JsonTool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "Sql format".to_string(),
            Box::new(SqlTool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "Datetime Convertor".to_string(),
            Box::new(DateTimeTool::default()) as Box<dyn Tool>,
        );
        tools.insert(
            "Hex Tool".to_string(),
            Box::new(HexTool::default()) as Box<dyn Tool>,
        );

        Self {
            selected_tool: None,
            search_text: String::new(),
            tools,
            dark_mode: true,
        }
    }

    pub fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Developer Tools").text_style(TextStyle::Name("logo".into())));
            ui.separator();

            // 工具列表区域
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for (id, tool) in self.tools.iter() {
                    if ui
                        .selectable_label(
                            self.selected_tool.as_ref().map_or(false, |s| s == id),
                            tool.name(),
                        )
                        .clicked()
                    {
                        self.selected_tool = Some(id.clone());
                    }
                    ui.add_space(10.0);
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
            if let Some(tool_id) = &self.selected_tool {
                if let Some(tool) = self.tools.get_mut(tool_id) {
                    tool.render(ui);
                }
            }
        });
    }
}

impl eframe::App for DeveloperTools {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            self.render_sidebar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_panel(ui);
        });
    }
}
