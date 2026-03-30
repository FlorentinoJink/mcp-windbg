//! Benchmarks for mcp-windbg-rs
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn bench_find_cdb_executable(c: &mut Criterion) {
    c.bench_function("find_cdb_executable (default)", |b| {
        b.iter(|| {
            black_box(mcp_windbg_rs::utils::find_cdb_executable(None));
        })
    });

    c.bench_function("find_cdb_executable (custom nonexistent)", |b| {
        b.iter(|| {
            black_box(mcp_windbg_rs::utils::find_cdb_executable(Some(Path::new(
                "C:\\nonexistent\\cdb.exe",
            ))));
        })
    });
}

fn bench_params_validate(c: &mut Criterion) {
    use mcp_windbg_rs::types::RunWindbgCmdParams;

    let valid_params = RunWindbgCmdParams {
        dump_path: Some("test.dmp".to_string()),
        connection_string: None,
        program_path: None,
        command: "k".to_string(),
    };

    let invalid_params = RunWindbgCmdParams {
        dump_path: None,
        connection_string: None,
        program_path: None,
        command: "k".to_string(),
    };

    c.bench_function("RunWindbgCmdParams::validate (valid)", |b| {
        b.iter(|| {
            black_box(valid_params.validate()).unwrap();
        })
    });

    c.bench_function("RunWindbgCmdParams::validate (invalid)", |b| {
        b.iter(|| {
            let _ = black_box(invalid_params.validate());
        })
    });
}

fn bench_tool_response_creation(c: &mut Criterion) {
    use mcp_windbg_rs::types::ToolResponse;

    c.bench_function("ToolResponse::text (short)", |b| {
        b.iter(|| {
            black_box(ToolResponse::text("hello"));
        })
    });

    c.bench_function("ToolResponse::text (1KB)", |b| {
        let text = "x".repeat(1024);
        b.iter(|| {
            black_box(ToolResponse::text(text.clone()));
        })
    });

    c.bench_function("ToolResponse::texts (100 lines)", |b| {
        let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
        b.iter(|| {
            black_box(ToolResponse::texts(lines.clone()));
        })
    });
}

fn bench_params_deserialization(c: &mut Criterion) {
    use mcp_windbg_rs::types::LaunchDebugParams;

    let json_minimal = r#"{"program_path": "app.exe"}"#;
    let json_full = r#"{
        "program_path": "C:\\test\\app.exe",
        "arguments": ["--flag", "value"],
        "working_directory": "C:\\test",
        "symbols_path": "C:\\symbols",
        "source_path": "C:\\src",
        "include_stack_trace": true,
        "include_modules": true
    }"#;

    c.bench_function("LaunchDebugParams deserialize (minimal)", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<LaunchDebugParams>(json_minimal).unwrap());
        })
    });

    c.bench_function("LaunchDebugParams deserialize (full)", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<LaunchDebugParams>(json_full).unwrap());
        })
    });
}

fn bench_dump_file_search(c: &mut Criterion) {
    let temp = tempfile::TempDir::new().unwrap();
    // Create 50 files, 10 of which are .dmp
    for i in 0..50 {
        let name = if i % 5 == 0 {
            format!("file_{}.dmp", i)
        } else {
            format!("file_{}.txt", i)
        };
        std::fs::write(temp.path().join(name), b"data").unwrap();
    }

    let path = temp.path().to_path_buf();
    c.bench_function("find_dump_files (50 files, 10 dumps)", |b| {
        b.iter(|| {
            black_box(mcp_windbg_rs::utils::find_dump_files(&path, false).unwrap());
        })
    });
}

fn bench_server_list_tools(c: &mut Criterion) {
    use mcp_windbg_rs::server::{McpServer, ServerConfig};

    let server = McpServer::new(ServerConfig::default());

    c.bench_function("McpServer::list_tools", |b| {
        b.iter(|| {
            black_box(server.list_tools());
        })
    });
}

criterion_group!(
    benches,
    bench_find_cdb_executable,
    bench_params_validate,
    bench_tool_response_creation,
    bench_params_deserialization,
    bench_dump_file_search,
    bench_server_list_tools,
);
criterion_main!(benches);
