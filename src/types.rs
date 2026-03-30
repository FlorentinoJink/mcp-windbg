//! MCP 协议和工具参数的共享类型定义
//!
//! 本模块包含用于 MCP 通信和工具参数定义的所有数据结构。

use serde::{Deserialize, Serialize};

/// MCP 工具响应
#[derive(Debug, Serialize, Clone)]
pub struct ToolResponse {
    /// 响应内容列表
    pub content: Vec<ContentItem>,
}

impl ToolResponse {
    /// 创建包含单个文本内容的响应
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![ContentItem::Text { text: text.into() }],
        }
    }

    /// 创建包含多个文本内容的响应
    pub fn texts(texts: Vec<String>) -> Self {
        Self {
            content: texts
                .into_iter()
                .map(|text| ContentItem::Text { text })
                .collect(),
        }
    }
}

/// 内容项类型
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ContentItem {
    /// 文本内容
    #[serde(rename = "text")]
    Text { text: String },
}

/// MCP 工具定义
#[derive(Debug, Serialize, Clone)]
pub struct ToolDefinition {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 输入参数的 JSON Schema
    pub input_schema: serde_json::Value,
}

/// open_windbg_dump 工具的参数
#[derive(Debug, Deserialize)]
pub struct OpenWindbgDumpParams {
    /// 转储文件路径
    pub dump_path: String,
    /// 是否包含堆栈跟踪
    #[serde(default)]
    pub include_stack_trace: bool,
    /// 是否包含模块信息
    #[serde(default)]
    pub include_modules: bool,
    /// 是否包含线程信息
    #[serde(default)]
    pub include_threads: bool,
}

/// open_windbg_remote 工具的参数
#[derive(Debug, Deserialize)]
pub struct OpenWindbgRemoteParams {
    /// 远程连接字符串 (例如: tcp:Port=5005,Server=192.168.0.100)
    pub connection_string: String,
    /// 是否包含堆栈跟踪
    #[serde(default)]
    pub include_stack_trace: bool,
    /// 是否包含模块信息
    #[serde(default)]
    pub include_modules: bool,
    /// 是否包含线程信息
    #[serde(default)]
    pub include_threads: bool,
}

/// run_windbg_cmd 工具的参数
#[derive(Debug, Deserialize)]
pub struct RunWindbgCmdParams {
    /// 转储文件路径（与 connection_string、program_path 互斥）
    pub dump_path: Option<String>,
    /// 远程连接字符串（与 dump_path、program_path 互斥）
    pub connection_string: Option<String>,
    /// 目标程序路径（与 dump_path、connection_string 互斥）
    pub program_path: Option<String>,
    /// 要执行的 WinDbg 命令
    pub command: String,
}

impl RunWindbgCmdParams {
    /// 验证参数：确保 dump_path、connection_string、program_path 三者互斥且至少提供一个
    pub fn validate(&self) -> Result<(), String> {
        let count = [
            self.dump_path.is_some(),
            self.connection_string.is_some(),
            self.program_path.is_some(),
        ]
        .iter()
        .filter(|&&v| v)
        .count();

        match count {
            0 => Err("One of dump_path, connection_string, or program_path must be provided".to_string()),
            1 => Ok(()),
            _ => Err("dump_path, connection_string, and program_path are mutually exclusive".to_string()),
        }
    }

    /// 获取会话标识符（转储路径、连接字符串或程序路径）
    pub fn session_identifier(&self) -> Option<&str> {
        self.dump_path
            .as_deref()
            .or(self.connection_string.as_deref())
            .or(self.program_path.as_deref())
    }
}

/// launch_debug 工具的参数
#[derive(Debug, Deserialize)]
pub struct LaunchDebugParams {
    /// 目标程序路径（必填）
    pub program_path: String,
    /// 命令行参数（可选）
    #[serde(default)]
    pub arguments: Option<Vec<String>>,
    /// 工作目录（可选）
    pub working_directory: Option<String>,
    /// 调试符号路径（可选）
    pub symbols_path: Option<String>,
    /// 调试源文件路径（可选）
    pub source_path: Option<String>,
    /// 是否包含堆栈跟踪
    #[serde(default)]
    pub include_stack_trace: bool,
    /// 是否包含模块信息
    #[serde(default)]
    pub include_modules: bool,
}

/// close_debug 工具的参数
#[derive(Debug, Deserialize)]
pub struct CloseDebugParams {
    /// 目标程序路径
    pub program_path: String,
}

/// close_windbg_dump 工具的参数
#[derive(Debug, Deserialize)]
pub struct CloseWindbgDumpParams {
    /// 要关闭的转储文件路径
    pub dump_path: String,
}

/// close_windbg_remote 工具的参数
#[derive(Debug, Deserialize)]
pub struct CloseWindbgRemoteParams {
    /// 要关闭的远程连接字符串
    pub connection_string: String,
}

/// list_windbg_dumps 工具的参数
#[derive(Debug, Deserialize)]
pub struct ListWindbgDumpsParams {
    /// 要搜索的目录路径（可选，默认使用系统转储目录）
    pub directory_path: Option<String>,
    /// 是否递归搜索子目录
    #[serde(default)]
    pub recursive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_response_text() {
        let response = ToolResponse::text("test message");
        assert_eq!(response.content.len(), 1);
        match &response.content[0] {
            ContentItem::Text { text } => assert_eq!(text, "test message"),
        }
    }

    #[test]
    fn test_tool_response_texts() {
        let response = ToolResponse::texts(vec!["msg1".to_string(), "msg2".to_string()]);
        assert_eq!(response.content.len(), 2);
    }

    #[test]
    fn test_run_windbg_cmd_params_validate() {
        // 三者都没有提供
        let params = RunWindbgCmdParams {
            dump_path: None,
            connection_string: None,
            program_path: None,
            command: "test".to_string(),
        };
        assert!(params.validate().is_err());

        // dump_path 和 connection_string 都提供了
        let params = RunWindbgCmdParams {
            dump_path: Some("test.dmp".to_string()),
            connection_string: Some("tcp:Port=5005".to_string()),
            program_path: None,
            command: "test".to_string(),
        };
        assert!(params.validate().is_err());

        // 只提供 dump_path
        let params = RunWindbgCmdParams {
            dump_path: Some("test.dmp".to_string()),
            connection_string: None,
            program_path: None,
            command: "test".to_string(),
        };
        assert!(params.validate().is_ok());

        // 只提供 connection_string
        let params = RunWindbgCmdParams {
            dump_path: None,
            connection_string: Some("tcp:Port=5005".to_string()),
            program_path: None,
            command: "test".to_string(),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_run_windbg_cmd_params_session_identifier() {
        let params = RunWindbgCmdParams {
            dump_path: Some("test.dmp".to_string()),
            connection_string: None,
            program_path: None,
            command: "test".to_string(),
        };
        assert_eq!(params.session_identifier(), Some("test.dmp"));

        let params = RunWindbgCmdParams {
            dump_path: None,
            connection_string: Some("tcp:Port=5005".to_string()),
            program_path: None,
            command: "test".to_string(),
        };
        assert_eq!(params.session_identifier(), Some("tcp:Port=5005"));
    }

    #[test]
    fn test_deserialize_open_windbg_dump_params() {
        let json = r#"{
            "dump_path": "C:\\dumps\\test.dmp",
            "include_stack_trace": true,
            "include_modules": false
        }"#;
        let params: OpenWindbgDumpParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.dump_path, "C:\\dumps\\test.dmp");
        assert!(params.include_stack_trace);
        assert!(!params.include_modules);
        assert!(!params.include_threads); // 默认值
    }

    #[test]
    fn test_deserialize_list_windbg_dumps_params() {
        let json = r#"{"recursive": true}"#;
        let params: ListWindbgDumpsParams = serde_json::from_str(json).unwrap();
        assert!(params.directory_path.is_none());
        assert!(params.recursive);
    }

    #[test]
    fn test_deserialize_launch_debug_params_full() {
        let json = r#"{
            "program_path": "C:\\test\\app.exe",
            "arguments": ["--flag", "value"],
            "working_directory": "C:\\test",
            "symbols_path": "C:\\symbols",
            "source_path": "C:\\src",
            "include_stack_trace": true,
            "include_modules": true
        }"#;
        let params: LaunchDebugParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.program_path, "C:\\test\\app.exe");
        assert_eq!(params.arguments.as_ref().unwrap(), &["--flag", "value"]);
        assert_eq!(params.working_directory.as_deref(), Some("C:\\test"));
        assert_eq!(params.symbols_path.as_deref(), Some("C:\\symbols"));
        assert_eq!(params.source_path.as_deref(), Some("C:\\src"));
        assert!(params.include_stack_trace);
        assert!(params.include_modules);
    }

    #[test]
    fn test_deserialize_launch_debug_params_minimal() {
        let json = r#"{"program_path": "app.exe"}"#;
        let params: LaunchDebugParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.program_path, "app.exe");
        assert!(params.arguments.is_none());
        assert!(params.working_directory.is_none());
        assert!(params.symbols_path.is_none());
        assert!(params.source_path.is_none());
        assert!(!params.include_stack_trace);
        assert!(!params.include_modules);
    }

    #[test]
    fn test_deserialize_close_debug_params() {
        let json = r#"{"program_path": "C:\\test\\app.exe"}"#;
        let params: CloseDebugParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.program_path, "C:\\test\\app.exe");
    }

    #[test]
    fn test_serialize_tool_response() {
        let response = ToolResponse::text("test output");
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"type\":\"text\""));
        assert!(json.contains("\"text\":\"test output\""));
    }
}
