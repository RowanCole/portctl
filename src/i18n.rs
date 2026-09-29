// 语言通过 feature 选择：默认中文（zh），英文用 --no-default-features --features en 构建
#[cfg(all(feature = "zh", feature = "en"))]
compile_error!("features `zh` and `en` are mutually exclusive");

#[cfg(not(any(feature = "zh", feature = "en")))]
compile_error!("enable one of the features: `zh` or `en`");

#[cfg(feature = "zh")]
mod texts {
    pub const ABOUT: &str = "查看并管理端口占用的跨平台工具（Linux/macOS/Windows）";
    pub const AFTER_HELP: &str = "\
示例:
  portctl find *:8080               查找所有 IP 上 8080 端口的 TCP/UDP 占用
  portctl find :3000                省略 ip，等同 *:3000
  portctl find 127.0.0.1:8080/TCP   查找指定 IP 的 TCP 占用
  portctl find *:53/UDP             查找 53 端口的 UDP 占用
  portctl find '[::1]:8080/UDP'     IPv6 地址需加方括号
  portctl kill *:8080               杀死占用 8080 端口的进程（TCP+UDP 一起）
  portctl kill *:8080/TCP           只杀死 TCP 占用者
  portctl list                      列出全部端口使用情况

提示: /协议 可省略，省略时表示 TCP+UDP 全部；ip、端口、协议均可写 *。
     ip 也可省略，:3000 等同 *:3000，匹配所有 IP。
     在 shell 中裸写 * 可能被展开为文件名，必要时请加引号，如 '*:8080'。";
    pub const KILL_ABOUT: &str = "杀死占用指定端口的进程";
    pub const FIND_ABOUT: &str = "查找占用指定端口的进程";
    pub const LIST_ABOUT: &str = "列出所有端口使用情况";
    pub const TARGET_HELP: &str = "目标，格式 [ip:端口][/协议]，如 :3000/TCP；ip 可省略（:3000 即所有 IP）或写 *，/协议 可省略（省略为 *）";
    pub const HEADERS: [&str; 6] = ["PID", "进程名", "协议", "本地地址", "远程地址", "状态"];
    pub const UNKNOWN: &str = "未知";

    pub fn unsupported_proto(other: &str) -> String {
        format!("不支持的协议: {other}，仅支持 TCP/UDP/*")
    }

    pub fn invalid_addr_v6(s: &str) -> String {
        format!("无效地址: {s}，IPv6 请写作 [::1]:8080")
    }

    pub fn invalid_addr(s: &str) -> String {
        format!("无效地址: {s}")
    }

    pub fn invalid_addr_format(s: &str) -> String {
        format!("无效地址: {s}，格式应为 [ip:端口][/协议]，例如 127.0.0.1:8080/TCP 或 *:8080")
    }

    pub fn invalid_ip(s: &str) -> String {
        format!("无效 IP: {s}")
    }

    pub fn invalid_port(s: &str) -> String {
        format!("无效端口: {s}")
    }

    pub fn fetch_sockets_failed(e: &str) -> String {
        format!("获取端口信息失败: {e}")
    }

    pub fn not_found(target: &str) -> String {
        format!("未找到占用 {target} 的进程")
    }

    pub fn unexpected_arg(arg: &str, suggestion: Option<&str>) -> String {
        let tip = match suggestion {
            Some(s) => format!("；是不是想用 '{s}'？"),
            None => "；运行 'portctl --help' 查看用法".to_string(),
        };
        format!("无法识别的参数: {arg}{tip}")
    }

    pub fn unexpected_subcommand(cmd: &str, suggestion: Option<&str>) -> String {
        let tip = match suggestion {
            Some(s) => format!("；是不是想用 '{s}'？"),
            None => "；运行 'portctl --help' 查看用法".to_string(),
        };
        format!("无法识别的子命令: {cmd}{tip}")
    }

    pub fn cannot_determine_pid() -> String {
        "无法确定占用端口的进程 PID，可能需要管理员/root 权限".to_string()
    }

    pub fn killed(pid: u32, name: &str) -> String {
        format!("已杀死 PID={pid} 进程名={name}")
    }

    pub fn kill_failed(pid: u32, name: &str, err: &str) -> String {
        format!("杀死 PID={pid}（{name}）失败: {err}")
    }

    pub fn kill_partial(failed: usize) -> String {
        format!("有 {failed} 个进程未能杀死")
    }
}

#[cfg(feature = "en")]
mod texts {
    pub const ABOUT: &str =
        "Cross-platform tool to inspect and manage port occupancy (Linux/macOS/Windows)";
    pub const AFTER_HELP: &str = "\
Examples:
  portctl find *:8080               find what occupies port 8080 (any IP, TCP/UDP)
  portctl find :3000                omit ip, same as *:3000
  portctl find 127.0.0.1:8080/TCP   find the TCP occupant on a specific IP
  portctl find *:53/UDP             find the UDP occupant of port 53
  portctl find '[::1]:8080/UDP'     bracket IPv6 addresses
  portctl kill *:8080               kill the occupant(s) of port 8080 (TCP+UDP)
  portctl kill *:8080/TCP           kill only the TCP occupant
  portctl list                      list all port usage

Tips: /protocol is optional; omitted or empty means both TCP and UDP.
     ip, port and protocol accept * as a wildcard.
     Omitting ip also works: :3000 is the same as *:3000.
     Quote arguments containing * in shells, e.g. '*:8080'.";
    pub const KILL_ABOUT: &str = "Kill the process occupying the given port";
    pub const FIND_ABOUT: &str = "Find the process occupying the given port";
    pub const LIST_ABOUT: &str = "List all port usage";
    pub const TARGET_HELP: &str = "Target in [ip:port][/protocol] form, e.g. :3000/TCP; omit ip (:3000) or use * to match all IPs, /protocol is optional (defaults to *)";
    pub const HEADERS: [&str; 6] = ["PID", "PROCESS", "PROTO", "LOCAL", "REMOTE", "STATE"];
    pub const UNKNOWN: &str = "unknown";

    pub fn unsupported_proto(other: &str) -> String {
        format!("Unsupported protocol: {other}, only TCP/UDP/* are supported")
    }

    pub fn invalid_addr_v6(s: &str) -> String {
        format!("Invalid address: {s}, write IPv6 as [::1]:8080")
    }

    pub fn invalid_addr(s: &str) -> String {
        format!("Invalid address: {s}")
    }

    pub fn invalid_addr_format(s: &str) -> String {
        format!("Invalid address: {s}, expected [ip:port][/protocol], e.g. 127.0.0.1:8080/TCP or *:8080")
    }

    pub fn invalid_ip(s: &str) -> String {
        format!("Invalid IP: {s}")
    }

    pub fn invalid_port(s: &str) -> String {
        format!("Invalid port: {s}")
    }

    pub fn fetch_sockets_failed(e: &str) -> String {
        format!("Failed to get socket info: {e}")
    }

    pub fn not_found(target: &str) -> String {
        format!("No process found occupying {target}")
    }

    pub fn unexpected_arg(arg: &str, suggestion: Option<&str>) -> String {
        let tip = match suggestion {
            Some(s) => format!("; did you mean '{s}'?"),
            None => "; run 'portctl --help' for usage".to_string(),
        };
        format!("Unrecognized argument: {arg}{tip}")
    }

    pub fn unexpected_subcommand(cmd: &str, suggestion: Option<&str>) -> String {
        let tip = match suggestion {
            Some(s) => format!("; did you mean '{s}'?"),
            None => "; run 'portctl --help' for usage".to_string(),
        };
        format!("Unrecognized subcommand: {cmd}{tip}")
    }

    pub fn cannot_determine_pid() -> String {
        "Cannot determine the PID owning the port, root/administrator may be required".to_string()
    }

    pub fn killed(pid: u32, name: &str) -> String {
        format!("Killed PID={pid} process={name}")
    }

    pub fn kill_failed(pid: u32, name: &str, err: &str) -> String {
        format!("Failed to kill PID={pid} ({name}): {err}")
    }

    pub fn kill_partial(failed: usize) -> String {
        format!("{failed} process(es) failed to be killed")
    }
}

pub use texts::*;
