# portctl

跨平台端口占用查看与管理工具，支持 Linux / macOS / Windows。

- **find**：查找占用指定端口的进程
- **kill**：杀死占用指定端口的进程
- **list**：列出系统所有端口使用情况
- 支持通配符、IPv6、TCP/UDP 协议筛选，查找结果以表格输出
- 通过 Cargo feature 支持中文 / 英文两种输出语言

## 构建

```bash
cargo build --release
```

二进制文件位于 `target/release/portctl`。也可以直接安装到 PATH：

```bash
cargo install --path .
```

切换输出语言：

```bash
cargo build                                        # 中文（默认）
cargo build --no-default-features --features en    # 英文
```

> `zh` 与 `en` 两个 feature 互斥，不能同时启用。

## 使用教程

### 命令格式

```
portctl find <ip:端口>[/协议]
portctl kill <ip:端口>[/协议]
portctl list
```

- **协议**：只能是 `TCP` / `UDP` / `*`（大小写不敏感），`*` 表示同时匹配两种协议
- **省略协议**：`/协议` 可省略，或写 `/`（`*:8080/`），效果等同 `*`
- **通配符 `*`**：
  - 用于 ip：匹配本机所有 IP（等同 `0.0.0.0`、`127.0.0.1`、局域网 IP、IPv6）
  - 用于端口：匹配 0-65535 任意端口
  - 用于协议：同时匹配 TCP 和 UDP
- **IPv6**：地址需加方括号，如 `[::1]:8080`

### find — 查找占用进程

```bash
portctl find *:8080               # 所有 IP 的 8080 端口，TCP+UDP 一起查
portctl find *:8080/TCP           # 只查 TCP
portctl find *:8080/UDP           # 只查 UDP
portctl find 127.0.0.1:8080/TCP   # 指定 IP + 指定协议
portctl find '[::1]:8080/UDP'     # IPv6 地址
```

输出（中文版）：

```
PID     进程名   协议  本地地址       远程地址   状态
------  -------  ----  -------------  ---------  ------
199261  python3  TCP   0.0.0.0:18096  0.0.0.0:0  LISTEN
```

英文版表头为 `PID / PROCESS / PROTO / LOCAL / REMOTE / STATE`。UDP 没有"远程地址"和"状态"概念，以 `-` 占位。

### kill — 杀死占用进程

```bash
portctl kill *:8080               # TCP+UDP 占用者一起杀
portctl kill *:8080/TCP           # 只杀 TCP 占用者
portctl kill 127.0.0.1:8080/UDP   # 只杀指定 IP 的 UDP 占用者
```

输出示例：

```
已杀死 PID=199261 进程名=python3
```

同一端口被多个进程占用时会逐个杀掉；全部成功返回退出码 0，有失败（如权限不足）则逐条报错并以非零退出码结束。

### list — 列出所有端口

```bash
portctl list
```

输出系统当前全部 TCP/UDP 连接与监听，格式同 find，按协议和本地地址排序。

### Shell 通配符注意事项

在 shell 中裸写 `*` 可能被展开成当前目录的文件名：

```bash
portctl find *:8080        # 安全：*:8080 匹配不到文件名，原样传递
portctl kill *:8080        # 同上
portctl find *:8080/*      # 不安全：末尾 * 可能被展开，建议加引号
portctl find '*:8080'      # 养成加引号的习惯最稳妥
```

### "未找到"提示格式

找不到进程时，提示中的目标会被规范化显示：通配 ip 显示为 `0.0.0.0`，指定协议时带后缀：

```
$ portctl find *:57336/*
未找到占用 0.0.0.0:57336 的进程
$ portctl find *:57336/TCP
未找到占用 0.0.0.0:57336/TCP 的进程
```

## 权限说明

- 查看其他用户进程的 PID / 进程名，或杀死它们，可能需要 **root / 管理员** 权限；权限不足时对应字段显示 `-` 或杀进程报错
- kill 使用 `SIGKILL`（Linux/macOS）/ `TerminateProcess`（Windows），进程无法拦截，直接终止

## 平台支持

| 平台   | 端口枚举方式        | 进程名获取                  | 杀进程              |
| ------ | ------------------- | --------------------------- | ------------------- |
| Linux  | procfs / netlink    | `/proc/{pid}/comm`          | `kill(SIGKILL)`     |
| macOS  | libproc             | `libproc::proc_pid::name`   | `kill(SIGKILL)`     |
| Windows| iphlpapi            | `QueryFullProcessImageNameW`| `TerminateProcess`  |

底层依赖 [netstat2](https://crates.io/crates/netstat2)，无需管理员权限即可查看基本端口信息。
