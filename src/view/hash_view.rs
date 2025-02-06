use egui::RichText;

use crate::model::HashAlgorithm;

use super::View;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashView {
    pub input: String,
}

impl HashView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl View for HashView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Hash Text");
            ui.separator();
            ui.label(RichText::new("Hash a text string using the function you need: MD5, SHA1, SHA256, SHA224, SHA512, SHA384, SHA3 or RIPEMD160").text_style(egui::TextStyle::Small));
            ui.add_space(15.0);
            ui.label("Input text:");
            ui.add_space(4.0);
            let text_edit = egui::TextEdit::multiline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(ui.available_width())
                .desired_rows(5)
                .hint_text("Enter text to hash...");
            ui.add(text_edit);

            ui.add_space(20.0);


            // hash result
            let column_widths = [60.0, ui.available_width() - 60.0, 60.0]; // 定义三列宽度
                  // 计算可用宽度（留出20px边距）
            let available_width = ui.available_width() - 120.0;
            // println!("available_width:{}",available_width);
            egui::Grid::new("hash result")
            .num_columns(2)
            .min_col_width(40.0) // 设置列最小宽度
            .max_col_width(available_width) // 设置最大宽度限制
            .spacing([10.0, 5.0]) // 增加水平间距
            // .striped(true)
            .show(ui, |ui| {
                self.hash_result_view(ui,&column_widths);
            });
        });
    }
}

impl HashView {
    fn hash_result_view(&mut self, ui: &mut egui::Ui, column_widths: &[f32]) {
        HashAlgorithm::iter().for_each(|hasher| {
            ui.label(format!("{}:", hasher.to_string()));

            // 第二列：哈希结果文本框
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let mut res = hasher.hash(&self.input);
                ui.add(
                    egui::TextEdit::singleline(&mut res)
                        .clip_text(true) // 允许文本溢出
                        .desired_width(column_widths[1] - 10.0) // 动态宽度
                        // .desired_width(f32::INFINITY) // 自动扩展
                        .font(egui::TextStyle::Monospace),
                );
            });

            // 第三列：复制按钮

            // ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            //     if ui
            //         .small_button(RichText::new("📋").text_style(TextStyle::Button))
            //         .clicked()
            //     {
            //         // 复制逻辑
            //     }
            // });

            // ui.with_layout(
            //     egui::Layout::right_to_left(egui::Align::Center),
            //     |ui| {
            //         if ui.button("📋").clicked() {
            //             // 实现复制逻辑
            //         }
            //     },
            // );
            ui.end_row();
        });
    }
}
