pub mod base64_tool;

pub mod calculator_tool;
pub mod datetime_tool;
pub mod diff_tool;
pub mod hash_tool;
pub mod hex_tool;
pub mod json_tool;
pub mod sql_tool;

use eframe::egui;

// 定义工具的状态管理特征
pub trait ToolState {
    fn clear(&mut self);
    fn is_dirty(&self) -> bool;
    fn mark_clean(&mut self);
}

// 定义工具的业务逻辑特征
pub trait ToolLogic {
    fn process(&mut self) -> Result<(), String>;
    fn validate(&self) -> bool;
}

// 定义工具的UI渲染特征
pub trait ToolUI {
    fn render(&mut self, ui: &mut egui::Ui);
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

// 定义完整的工具特征，组合以上特征
pub trait Tool: ToolState + ToolLogic + ToolUI {
    fn as_state(&self) -> &dyn ToolState;
    fn as_state_mut(&mut self) -> &mut dyn ToolState;
    fn as_logic(&self) -> &dyn ToolLogic;
    fn as_logic_mut(&mut self) -> &mut dyn ToolLogic;
    fn as_ui(&self) -> &dyn ToolUI;
    fn as_ui_mut(&mut self) -> &mut dyn ToolUI;
}

// 错误处理
#[derive(Debug)]
pub enum ToolError {
    ValidationError(String),
    ProcessError(String),
    StateError(String),
}

pub type ToolResult<T> = Result<T, ToolError>;
