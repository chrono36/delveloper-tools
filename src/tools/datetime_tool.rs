use super::{Tool, ToolLogic, ToolState, ToolUI};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use eframe::egui;

#[derive(Default)]
pub struct DateTimeToolState {
    input: String,
    output: String,
    error: String,
    is_dirty: bool,
    selected_format: String,
}

impl ToolState for DateTimeToolState {
    fn clear(&mut self) {
        self.input.clear();
        self.output.clear();
        self.error.clear();
        self.is_dirty = false;
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}

impl ToolLogic for DateTimeTool {
    fn process(&mut self) -> Result<(), String> {
        if !self.validate() {
            return Err("输入不能为空".to_string());
        }
        self.timestamp_to_datetime();
        if !self.state.error.is_empty() {
            return Err(self.state.error.clone());
        }
        Ok(())
    }

    fn validate(&self) -> bool {
        !self.state.input.trim().is_empty()
    }
}

impl ToolUI for DateTimeTool {
    fn render(&mut self, ui: &mut egui::Ui) {
        self.ui(ui);
    }

    fn name(&self) -> &str {
        "Date-time converter"
    }

    fn description(&self) -> &str {
        "Convert date and time into the various different formats"
    }
}

impl Tool for DateTimeTool {
    fn as_state(&self) -> &dyn ToolState {
        &self.state
    }

    fn as_state_mut(&mut self) -> &mut dyn ToolState {
        &mut self.state
    }

    fn as_logic(&self) -> &dyn ToolLogic {
        self
    }

    fn as_logic_mut(&mut self) -> &mut dyn ToolLogic {
        self
    }

    fn as_ui(&self) -> &dyn ToolUI {
        self
    }

    fn as_ui_mut(&mut self) -> &mut dyn ToolUI {
        self
    }
}

impl ToolState for DateTimeTool {
    fn clear(&mut self) {
        self.state.clear();
    }

    fn is_dirty(&self) -> bool {
        self.state.is_dirty()
    }

    fn mark_clean(&mut self) {
        self.state.mark_clean();
    }
}

pub struct DateTimeTool {
    state: DateTimeToolState,
}

impl Default for DateTimeTool {
    fn default() -> Self {
        Self {
            state: DateTimeToolState::default(),
        }
    }
}

impl DateTimeTool {
    pub fn new() -> Self {
        Self::default()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Date-time converter");
        ui.label("Convert date and time into the various different formats");
        ui.add_space(8.0);

        // 输入区域
        ui.horizontal(|ui| {
            let text_edit = egui::TextEdit::singleline(&mut self.state.input)
                .hint_text("Put your date string here...");
            if ui.add(text_edit).changed() {
                self.state.is_dirty = true;
            }

            // 添加格式选择下拉框
            egui::ComboBox::from_label("")
                .selected_text(&self.state.selected_format)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.state.selected_format,
                        "timestamp".to_string(),
                        "Timestamp",
                    );
                    ui.selectable_value(
                        &mut self.state.selected_format,
                        "datetime".to_string(),
                        "DateTime",
                    );
                });

            if ui.button("Convert").clicked() {
                match self.state.selected_format.as_str() {
                    "timestamp" => self.timestamp_to_datetime(),
                    "datetime" => self.datetime_to_timestamp(),
                    _ => self.state.error = "Invalid format selected".to_string(),
                }
            }

            if ui.button("Clear").clicked() {
                self.state.clear();
            }
        });

        ui.add_space(8.0);

        // 输出区域
        if !self.state.error.is_empty() {
            ui.colored_label(egui::Color32::RED, &self.state.error);
        } else if !self.state.input.is_empty() {
            if let Some(datetime) = self.parse_input() {
                // JS locale date string
                self.format_output_row(
                    ui,
                    "JS locale date string",
                    self.format_js_locale(&datetime),
                );

                // ISO 8601
                self.format_output_row(ui, "ISO 8601", self.format_iso8601(&datetime));

                // ISO 9075
                self.format_output_row(ui, "ISO 9075", self.format_iso9075(&datetime));

                // RFC 3339
                self.format_output_row(ui, "RFC 3339", self.format_rfc3339(&datetime));

                // RFC 7231
                self.format_output_row(ui, "RFC 7231", self.format_rfc7231(&datetime));

                // Unix timestamp
                self.format_output_row(ui, "Unix timestamp", self.format_unix_timestamp(&datetime));

                // Timestamp
                self.format_output_row(ui, "Timestamp", self.format_timestamp(&datetime));

                // UTC format
                self.format_output_row(ui, "UTC format", self.format_utc(&datetime));

                // Mongo ObjectID
                self.format_output_row(ui, "Mongo ObjectID", self.format_mongo_objectid(&datetime));

                // Excel date/time
                self.format_output_row(
                    ui,
                    "Excel date/time",
                    self.format_excel_datetime(&datetime),
                );
            }
        }
    }

    fn format_output_row(&mut self, ui: &mut egui::Ui, label: &str, content: String) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut output = content.clone();
            ui.add(
                egui::TextEdit::singleline(&mut output).desired_width(ui.available_width() - 50.0),
            );
            if ui.button("📋").clicked() {
                ui.output_mut(|o| o.copied_text = content);
            }
        });
    }

    fn parse_input(&self) -> Option<DateTime<Local>> {
        match self.state.selected_format.as_str() {
            "timestamp" => {
                if let Ok(timestamp) = self.state.input.trim().parse::<i64>() {
                    if let Some(datetime) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
                        Some(Local.from_utc_datetime(&datetime))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "datetime" => {
                if let Ok(datetime) =
                    NaiveDateTime::parse_from_str(self.state.input.trim(), "%Y-%m-%d %H:%M:%S")
                {
                    Some(Local.from_utc_datetime(&datetime))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn format_js_locale(&self, datetime: &DateTime<Local>) -> String {
        datetime.format("%a %b %d %Y %H:%M:%S GMT%z").to_string()
    }

    fn format_iso8601(&self, datetime: &DateTime<Local>) -> String {
        datetime.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    }

    fn format_iso9075(&self, datetime: &DateTime<Local>) -> String {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn format_rfc3339(&self, datetime: &DateTime<Local>) -> String {
        datetime.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
    }

    fn format_rfc7231(&self, datetime: &DateTime<Local>) -> String {
        datetime
            .with_timezone(&Utc)
            .format("%a, %d %b %Y %H:%M:%S GMT")
            .to_string()
    }

    fn format_unix_timestamp(&self, datetime: &DateTime<Local>) -> String {
        datetime.timestamp().to_string()
    }

    fn format_timestamp(&self, datetime: &DateTime<Local>) -> String {
        format!(
            "{}{:03}",
            datetime.timestamp(),
            datetime.timestamp_subsec_millis()
        )
    }

    fn format_utc(&self, datetime: &DateTime<Local>) -> String {
        datetime
            .with_timezone(&Utc)
            .format("%a, %d %b %Y %H:%M:%S GMT")
            .to_string()
    }

    fn format_mongo_objectid(&self, datetime: &DateTime<Local>) -> String {
        format!("{:x}000000000000000", (datetime.timestamp() as u32))
    }

    fn format_excel_datetime(&self, datetime: &DateTime<Local>) -> String {
        let days_since_1900 = (datetime.timestamp() as f64) / 86400.0 + 25569.0;
        format!("{:.10}", days_since_1900)
    }

    fn timestamp_to_datetime(&mut self) {
        self.state.error.clear();

        match self.state.input.trim().parse::<i64>() {
            Ok(timestamp) => {
                if let Some(datetime) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
                    let local_time: DateTime<Local> = Local.from_utc_datetime(&datetime);
                    self.state.output = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
                } else {
                    self.state.error = "无效的时间戳".to_string();
                }
            }
            Err(_) => {
                self.state.error = "请输入有效的时间戳".to_string();
            }
        }
    }

    fn datetime_to_timestamp(&mut self) {
        self.state.error.clear();

        match NaiveDateTime::parse_from_str(self.state.input.trim(), "%Y-%m-%d %H:%M:%S") {
            Ok(datetime) => {
                let local_time = Local.from_utc_datetime(&datetime);
                self.state.output = local_time.timestamp().to_string();
            }
            Err(_) => {
                self.state.error = "请按照格式输入日期时间：YYYY-MM-DD HH:mm:SS".to_string();
            }
        }
    }
}
