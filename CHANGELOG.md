# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-03-31

### Added
- `launch_debug` tool — launch a program directly under CDB for interactive debugging
- `close_debug` tool — close a launch debug session and terminate the target program
- `program_path` parameter for `run_windbg_cmd` (three-way mutual exclusion with `dump_path` and `connection_string`)
- `symbols_path` and `source_path` parameters for `launch_debug`
- `_NT_SOURCE_PATH` environment variable support in server config
- Dynamic WinDbg Preview discovery (auto-detect any installed version from Microsoft Store)
- `winget install Microsoft.WinDbg` hint when CDB is not found

### Fixed
- CDB output with non-UTF-8 encoding (GBK/CP936 on Chinese Windows) no longer crashes the server — uses lossy UTF-8 conversion

### Changed
- Rewrote README (EN/CN) — removed roadmap, restructured as a proper open-source project
- Removed unused `config.example.toml`

## [0.1.2] - 2026-03-30

### Added
- Separate initialization timeout (default: 120s) for dump file loading and symbol downloads
- `MCP_WINDBG_INIT_TIMEOUT` environment variable and `--init-timeout` CLI flag
- Unique timestamp-based command markers to prevent output conflicts
- Output size limit (100k lines) to prevent memory overflow

### Fixed
- Timeout issues when opening large dump files (>300MB)
- Command execution failures after opening dump files
- Marker detection conflicts in command output

## [0.1.0] - 2026-03-29

### Added
- Initial release
- Crash dump analysis with `open_windbg_dump`
- Remote debugging with `open_windbg_remote`
- Custom command execution with `run_windbg_cmd`
- Session management with connection pooling
- MCP server with stdio transport
- CDB auto-discovery from Windows SDK paths
