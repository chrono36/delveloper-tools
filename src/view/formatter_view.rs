use egui::RichText;

use crate::model::{sql_formatter::SqlFormatter, JsonFormatter};

use super::View;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatterView {
    pub input: String,
    pub err_msg: String,
    formatter_type: FormatterType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FormatterType {
    JSON,
    SQL,
}

impl FormatterView {
    pub fn new(formatter_type: &str) -> Self {
        let formatter_type = match formatter_type {
            "json" => FormatterType::JSON,
            "sql" => FormatterType::SQL,
            _ => FormatterType::JSON,
        };

        Self {
            input: String::new(),
            err_msg: String::new(),
            formatter_type: formatter_type,
        }
    }

    pub fn title(&self) -> &str {
        match self.formatter_type {
            FormatterType::JSON => "Json Formatter",
            FormatterType::SQL => "Sql Formatter",
        }
    }

    pub fn language(&self) -> &str {
        match self.formatter_type {
            FormatterType::JSON => "json",
            FormatterType::SQL => "sql",
        }
    }

    pub fn description(&self) -> &str {
        match self.formatter_type {
            FormatterType::JSON => {
                "Prettify your JSON string into a friendly, human-readable format."
            }
            FormatterType::SQL => {
                "Format and prettify your SQL queries online (it supports various SQL dialects)."
            }
        }
    }

    pub fn hint_text(&self) -> &str {
        match self.formatter_type {
            FormatterType::JSON => "Enter your Json ...",
            FormatterType::SQL => "Enter your Sql ...",
        }
    }

    pub fn formatter(&mut self) {
        let res = match self.formatter_type {
            FormatterType::JSON => JsonFormatter::pretty_json(&self.input),
            FormatterType::SQL => SqlFormatter::formatter(&self.input),
        };

        match res {
            Ok(r) => self.input = r,
            Err(e) => self.err_msg = e.to_string(),
        }
    }
}

impl View for FormatterView {
    fn render(&mut self, ui: &mut egui::Ui) {
        let Self {
            input,
            err_msg,
            formatter_type,
        } = self;

        ui.vertical(|ui| {
            ui.heading(self.title());
            ui.separator();
            ui.label(RichText::new(self.description()).text_style(egui::TextStyle::Small));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui
                    .link(RichText::new("formatter").text_style(egui::TextStyle::Button))
                    .clicked()
                {
                    self.formatter();
                }
            });

            ui.add_space(10.0);

            let theme =
                egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

            let language = match self.formatter_type {
                FormatterType::JSON => "json",
                FormatterType::SQL => "sql",
            };

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &theme,
                    string,
                    language,
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            egui::ScrollArea::both().show(ui, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.input)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(20)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter);
                let _ = ui.add(text_edit);
            });

            // let text_edit = egui::TextEdit::multiline(&mut self.input)
            //     .font(egui::TextStyle::Monospace)
            //     .desired_rows(20)
            //     .desired_width(ui.available_width())
            //     .hint_text("Enter your format text ...")
            //     .lock_focus(true)
            //     .code_editor();
            // let _ = ui.add(text_edit);
        });
    }
}
