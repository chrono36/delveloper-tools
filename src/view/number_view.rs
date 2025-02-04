// number base converter

use egui::RichText;

use crate::model::NumberBaseConverter;

use super::View;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberBaseConverterView {
    pub input: String,
    pub base: NumberBaseConverter,
    pub num: i64,
}

impl NumberBaseConverterView {
    pub fn new() -> Self {
        Self {
            input: String::from("36"),
            base: NumberBaseConverter::Decimal,
            num: 36,
        }
    }

    fn convert_list(&mut self, ui: &mut egui::Ui) {
        let decimal = if self.input.is_empty() {
            0
        } else {
            self.base
                .to_decimal(self.input.as_str())
                .unwrap_or_default()
        };

        NumberBaseConverter::iter().for_each(|converter| {
            ui.label(RichText::new(converter.to_string()));
            let convert_str = converter.convert(decimal);
            ui.text_edit_singleline(&mut convert_str.unwrap());
            ui.end_row();
        });
    }
}

impl View for NumberBaseConverterView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui|{
            ui.heading("Number base Converter");
            ui.separator();
            ui.label(RichText::new("Convert a number between different bases (decimal, hexadecimal, binary, octal, base64, ...)").text_style(egui::TextStyle::Small));

            ui.add_space(20.0);

            ui.horizontal(|ui|{
                // text edit input number
                let text_edit = egui::TextEdit::singleline(&mut self.input)
                .hint_text("PInput number here...");

                let _response = ui.add(text_edit);
                ui.add_space(15.0);
                    // 添加格式选择下拉框
                    egui::ComboBox::from_label("")
                    .selected_text(&self.base.to_string())
                    .show_ui(ui, |ui| {
                        NumberBaseConverter::iter()
                        .for_each(|converter|{
                            ui.selectable_value(&mut self.base, *converter, converter.to_string());
                        });
                    });
                });
                 ui.add_space(15.0);
                // formater list
                let _column_widths = [60.0, ui.available_width() - 60.0, 60.0]; // 定义三列宽度
                // 计算可用宽度（留出20px边距）
                let available_width = ui.available_width() - 100.0;
                // println!("available_width:{}",available_width);
                egui::Grid::new("formatter list")
                .num_columns(2)
                .min_col_width(40.0) // 设置列最小宽度
                .max_col_width(available_width) // 设置最大宽度限制
                .spacing([10.0, 5.0]) // 增加水平间距
                .show(ui, |ui| {
                self.convert_list(ui);
                });
        });
    }
}
