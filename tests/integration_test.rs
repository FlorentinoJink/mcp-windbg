//! Integration tests for mcp-windbg-rs
//!
//! These tests verify end-to-end behavior of the MCP server components.
//! Tests that require CDB are gated behind a check for CDB availability.

use mcp_windbg_rs::session::SessionManager;
use mcp_windbg_rs::tools;
use mcp_windbg_rs::types::*;
use mcp_windbg_rs::utils;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

fn has_cdb() -> bool {
    utils::find_cdb_executable(None).is_some()
}

// --- Session Manager Integration Tests ---

#[tokio::test]
async fn test_session_manager_dump_not_found() {
    let manager = SessionManager::new(Duration::from_secs(5), Duration::from_secs(10), false);
    let result = manager
        .get_or_create_dump_session(Path::new("nonexistent.dmp"), None, None)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_manager_launch_not_found() {
    let manager = SessionManager::new(Duration::from_secs(5), Duration::from_secs(10), false);
    let result = manager
        .get_or_create_launch_session(
            Path::new("nonexistent.exe"),
            None, None, None, None, None,
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_manager_close_nonexistent() {
    let manager = SessionManager::new(Duration::from_secs(5), Duration::from_secs(10), false);
    let result = manager.close_session("does_not_exist").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_manager_close_all_empty() {
    let manager = SessionManager::new(Duration::from_secs(5), Duration::from_secs(10), false);
    assert_eq!(manager.active_session_count().await, 0);
    let result = manager.close_all_sessions().await;
    assert!(result.is_ok());
}

// --- Tool Handler Integration Tests ---

#[tokio::test]
async fn test_handle_launch_debug_nonexistent_program() {
    let manager = Arc::new(SessionManager::new(
        Duration::from_secs(5), Duration::from_secs(10), false,
    ));
    let params = LaunchDebugParams {
        program_path: "C:\\definitely\\not\\a\\real\\program.exe".to_string(),
        arguments: None,
        working_directory: None,
        symbols_path: None,
        source_path: None,
        include_stack_trace: false,
        include_modules: false,
    };
    let result = tools::handle_launch_debug(manager, params, None, None, None).await;
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("not\\a\\real\\program.exe"));
}

#[tokio::test]
async fn test_handle_close_debug_nonexistent_session() {
    let manager = Arc::new(SessionManager::new(
        Duration::from_secs(5), Duration::from_secs(10), false,
    ));
    let params = CloseDebugParams {
        program_path: "C:\\nonexistent\\app.exe".to_string(),
    };
    let result = tools::handle_close_debug(manager, params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handle_run_cmd_no_session_identifier() {
    let manager = Arc::new(SessionManager::new(
        Duration::from_secs(5), Duration::from_secs(10), false,
    ));
    let params = RunWindbgCmdParams {
        dump_path: None,
        connection_string: None,
        program_path: None,
        command: "k".to_string(),
    };
    let result = tools::handle_run_windbg_cmd(manager, params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handle_list_dumps_nonexistent_dir() {
    let params = ListWindbgDumpsParams {
        directory_path: Some("C:\\this\\dir\\does\\not\\exist".to_string()),
        recursive: false,
    };
    let result = tools::handle_list_windbg_dumps(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handle_list_dumps_temp_dir() {
    let temp = tempfile::TempDir::new().unwrap();
    let params = ListWindbgDumpsParams {
        directory_path: Some(temp.path().to_string_lossy().to_string()),
        recursive: false,
    };
    let result = tools::handle_list_windbg_dumps(params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    match &response.content[0] {
        ContentItem::Text { text } => {
            assert!(text.contains("No dump files found"));
        }
    }
}

// --- CDB-dependent tests (skipped if CDB not installed) ---

#[tokio::test]
async fn test_cdb_discovery() {
    if !has_cdb() {
        eprintln!("Skipping: CDB not found");
        return;
    }
    let cdb = utils::find_cdb_executable(None).unwrap();
    assert!(cdb.exists());
    assert!(cdb.to_string_lossy().to_lowercase().contains("cdb.exe"));
}

// --- Utils Integration Tests ---

#[test]
fn test_find_dump_files_in_temp_dir() {
    let temp = tempfile::TempDir::new().unwrap();
    std::fs::write(temp.path().join("crash.dmp"), b"fake dump").unwrap();
    std::fs::write(temp.path().join("notes.txt"), b"not a dump").unwrap();

    let files = utils::find_dump_files(temp.path(), false).unwrap();
    assert_eq!(files.len(), 1);
    assert!(files[0].path.to_string_lossy().contains("crash.dmp"));
}

#[test]
fn test_find_dump_files_recursive_in_temp() {
    let temp = tempfile::TempDir::new().unwrap();
    let sub = temp.path().join("subdir");
    std::fs::create_dir(&sub).unwrap();
    std::fs::write(temp.path().join("a.dmp"), b"dump1").unwrap();
    std::fs::write(sub.join("b.dmp"), b"dump2").unwrap();

    let flat = utils::find_dump_files(temp.path(), false).unwrap();
    assert_eq!(flat.len(), 1);

    let recursive = utils::find_dump_files(temp.path(), true).unwrap();
    assert_eq!(recursive.len(), 2);
}
