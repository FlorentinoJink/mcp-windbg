# mcp-windbg-rs

中文 | [English](./README.md)

一个 [Model Context Protocol](https://modelcontextprotocol.io/) (MCP) 服务器，用于 Windows 调试 — 崩溃转储分析、远程调试和直接程序调试，基于 CDB 实现。

使用 Rust 和 [Tokio](https://tokio.rs/) 构建，编译为单一可执行文件，无运行时依赖。

## 功能

- **崩溃转储分析** — 打开 `.dmp` 文件，自动执行 `!analyze -v`，查看线程、模块和堆栈
- **远程调试** — 通过连接字符串连接远程调试会话
- **直接程序调试** — 在 CDB 下启动程序，设置断点、单步执行、查看变量
- **会话管理** — 支持多个并发调试会话，自动复用已有会话
- **可配置超时** — 初始化超时（符号加载）和命令执行超时分别配置

## 前置要求

- Windows 10+
- [Debugging Tools for Windows](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/)（提供 `cdb.exe`）

如果未安装 CDB，可以通过以下命令安装：

```bash
winget install Microsoft.WinDbg
```

服务器会自动从 Windows SDK 默认路径、WinDbg Preview（Microsoft Store）和 `PATH` 中查找 CDB。

## 安装

### 从源码构建

```bash
cargo build --release
```

可执行文件：`target/release/mcp-windbg-rs.exe`

## 配置

### VS Code / Kiro

`.vscode/mcp.json`：

```json
{
  "servers": {
    "mcp-windbg": {
      "type": "stdio",
      "command": "/path/to/mcp-windbg-rs.exe",
      "args": [],
      "env": {
        "_NT_SYMBOL_PATH": "SRV*C:\\Symbols*https://msdl.microsoft.com/download/symbols"
      }
    }
  }
}
```

### Claude Desktop / Cline / 其他 MCP 客户端

```json
{
  "mcpServers": {
    "mcp-windbg-rs": {
      "command": "mcp-windbg-rs",
      "args": [],
      "env": {
        "_NT_SYMBOL_PATH": "SRV*C:\\Symbols*https://msdl.microsoft.com/download/symbols"
      }
    }
  }
}
```

### 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `CDB_PATH` | 自定义 `cdb.exe` 路径 | 自动查找 |
| `_NT_SYMBOL_PATH` | 调试符号搜索路径 | — |
| `_NT_SOURCE_PATH` | 源文件搜索路径 | — |
| `MCP_WINDBG_TIMEOUT` | 命令执行超时（秒） | `30` |
| `MCP_WINDBG_INIT_TIMEOUT` | 初始化超时，用于 dump 加载和符号下载（秒） | `120` |
| `MCP_WINDBG_VERBOSE` | 详细日志（`true`/`false`） | `false` |

### 命令行选项

```
mcp-windbg-rs [选项]

  --timeout <秒数>          命令执行超时（默认：30）
  --init-timeout <秒数>     初始化超时（默认：120）
  --verbose                 启用详细日志
```

## 工具列表

| 工具 | 说明 |
|------|------|
| `open_windbg_dump` | 打开并分析崩溃转储文件 |
| `open_windbg_remote` | 连接远程调试会话 |
| `launch_debug` | 在 CDB 下启动程序进行调试 |
| `run_windbg_cmd` | 在会话中执行任意 WinDbg/CDB 命令 |
| `close_windbg_dump` | 关闭转储文件会话 |
| `close_windbg_remote` | 关闭远程调试会话 |
| `close_debug` | 关闭调试会话并终止目标程序 |
| `list_windbg_dumps` | 列出目录中的 `.dmp` 文件 |

## 使用示例

### 崩溃转储分析

```
分析 C:\dumps\app.dmp 这个崩溃转储文件
```

### 远程调试

```
连接到 tcp:Port=5005,Server=192.168.0.100 并显示当前状态
```

### 直接调试程序

启动程序、设置断点、单步执行：

```
启动 C:\MyApp\app.exe 进行调试
```

然后用 `run_windbg_cmd` 控制执行：

```
bp main          — 在 main 设置断点
g                — 继续执行
p                — 单步跳过
t                — 单步进入
k                — 查看堆栈
dv               — 查看局部变量
lsa .            — 显示当前位置的源码
```

`launch_debug` 工具支持以下可选参数：

| 参数 | 类型 | 说明 |
|------|------|------|
| `program_path` | string | 目标程序路径（必填） |
| `arguments` | string[] | 命令行参数 |
| `working_directory` | string | 工作目录 |
| `symbols_path` | string | PDB 符号搜索路径 |
| `source_path` | string | 源文件路径，用于源码级调试 |
| `include_stack_trace` | boolean | 包含初始堆栈跟踪 |
| `include_modules` | boolean | 包含已加载模块列表 |

### 关闭会话

```
关闭 C:\MyApp\app.exe 的调试会话
```

## 故障排除

**找不到 CDB** — 执行 `winget install Microsoft.WinDbg` 安装，或设置 `CDB_PATH` 指向 `cdb.exe`。

**符号加载失败** — 设置 `_NT_SYMBOL_PATH`，推荐值：`SRV*C:\Symbols*https://msdl.microsoft.com/download/symbols`

**命令超时** — 通过 `--timeout 60` 或 `MCP_WINDBG_TIMEOUT=60` 增加超时。大型 dump 和符号下载可能需要更高的 `MCP_WINDBG_INIT_TIMEOUT`。

## 相关链接

- [mcp-windbg (Python)](https://github.com/svnscha/mcp-windbg) — 原始 Python 实现
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [WinDbg 文档](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)

## 许可证

[AGPL-3.0-or-later](./LICENSE)
