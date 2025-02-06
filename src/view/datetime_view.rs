use crate::model::DateTimeConverter;

use super::View;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use egui::RichText;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimeConverterView {
    pub input: String,
    pub selected_format: String,
    pub number: u64,
    pub local_time: DateTime<Local>,
    pub err_msg: String,
}

impl DateTimeConverterView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_format: String::from("timestamp"),
            number: 0,
            err_msg: String::new(),
            local_time: Local::now(),
        }
    }

    fn parse(&mut self) {
        match self.selected_format.as_str() {
            "timestamp" => self.timestamp_to_datetime(),
            "datetime" => self.datetime_to_timestamp(),
            _ => (),
        };
    }

    fn timestamp_to_datetime(&mut self) {
        self.err_msg.clear();

        match self.input.trim().parse::<i64>() {
            Ok(timestamp) => {
                if let Some(datetime) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
                    let local_time: DateTime<Local> = Local.from_utc_datetime(&datetime);
                    self.local_time = local_time;
                } else {
                    self.err_msg = "Invalid timestamp ...".to_string();
                }
            }
            Err(_) => {
                self.err_msg = "Please enter a valid timestamp".to_string();
            }
        }
    }

    fn datetime_to_timestamp(&mut self) {
        self.err_msg.clear();

        match NaiveDateTime::parse_from_str(self.input.trim(), "%Y-%m-%d %H:%M:%S") {
            Ok(datetime) => {
                let local_time = Local.from_utc_datetime(&datetime);
                self.local_time = local_time
            }
            Err(_) => {
                self.err_msg =
                    "Please enter the date and time in the format：YYYY-MM-DD HH:mm:SS".to_string();
            }
        }
    }

    fn formater_list(&self, ui: &mut egui::Ui) {
        DateTimeConverter::iter().for_each(|converter| {
            ui.label(converter.name());
            let mut formatter_str = converter.formatter(self.local_time);
            ui.text_edit_singleline(&mut formatter_str);

            // ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            //     // ui.set_width(60.0); // 固定按钮列宽
            //     if ui.small_button("📋").clicked() {
            //         // 复制逻辑
            //     }
            // });

            ui.end_row();
        });
    }
}

impl View for DateTimeConverterView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Date-time converter");
            ui.separator();
            ui.label(
                RichText::new("Convert date and time into the various different formats")
                    .text_style(egui::TextStyle::Small),
            );

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.input)
                    .hint_text("Put your date string here...");

                let response = ui.add(text_edit);

                if response.changed() || response.lost_focus() {
                    self.parse();
                }

                if self.input.is_empty() {
                    self.local_time = Local::now();
                }

                // 添加格式选择下拉框
                egui::ComboBox::from_label("")
                    .selected_text(&self.selected_format)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_format,
                            "timestamp".to_string(),
                            "Timestamp",
                        );
                        ui.selectable_value(
                            &mut self.selected_format,
                            "datetime".to_string(),
                            "DateTime",
                        );
                    });
            });

            ui.add_space(20.0);

            // 输出区域
            if !self.err_msg.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.err_msg);
            }

            // formater list
            let _column_widths = [60.0, ui.available_width() - 60.0, 60.0]; // 定义三列宽度
                                                                            // 计算可用宽度（留出20px边距）
            let available_width = ui.available_width() - 120.0;
            // println!("available_width:{}",available_width);
            egui::Grid::new("formatter list")
                .num_columns(2)
                .min_col_width(40.0) // 设置列最小宽度
                .max_col_width(available_width) // 设置最大宽度限制
                .spacing([10.0, 5.0]) // 增加水平间距
                // .striped(true)
                .show(ui, |ui| {
                    self.formater_list(ui);
                });
        });
    }
}
