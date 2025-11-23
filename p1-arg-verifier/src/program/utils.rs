// const
pub const ICON_SUCCESS: &str = "✅";
pub const ICON_ERROR: &str = "❌";
pub const MESSAGE_ARG_SUCCESS: &str = "Success: Param 1 exist ->";
pub const MESSAGE_ARG_ERROR: &str = "Error: Param 1 invalid or missing ->";
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const COMMAND_CARGO: &str = "cargo";

// enum
#[derive(Debug, PartialEq, Clone)]
pub enum MessageOutput {
    Success(String),
    Error(String),
}
