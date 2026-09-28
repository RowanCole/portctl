use std::net::IpAddr;

use clap::{Parser, Subcommand};

use crate::i18n;

#[derive(Parser)]
#[command(
    name = "portctl",
    version,
    about = i18n::ABOUT,
    after_help = i18n::AFTER_HELP
)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = i18n::KILL_ABOUT)]
    Kill {
        #[arg(help = i18n::TARGET_HELP)]
        target: String,
    },
    #[command(about = i18n::FIND_ABOUT)]
    Find {
        #[arg(help = i18n::TARGET_HELP)]
        target: String,
    },
    #[command(about = i18n::LIST_ABOUT)]
    List,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Proto {
    TCP,
    UDP,
    Any,
}

impl std::str::FromStr for Proto {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "TCP" => Ok(Proto::TCP),
            "UDP" => Ok(Proto::UDP),
            "*" => Ok(Proto::Any),
            other => Err(i18n::unsupported_proto(other)),
        }
    }
}

// 目标：[ip:端口][/协议]，ip 与端口均可为 *
#[derive(Clone, Copy)]
pub struct Target {
    pub ip: Option<IpAddr>,
    pub port: Option<u16>,
}

pub fn parse_target(s: &str) -> Result<(Target, Proto), String> {
    // 协议跟在最后一个 / 之后，IP 地址本身不含 /
    let (addr_str, proto) = match s.rsplit_once('/') {
        Some((addr, proto)) => (addr, Some(proto)),
        None => (s, None),
    };
    let proto = parse_proto(proto)?;

    let (ip_str, port_str) = if let Some(rest) = addr_str.strip_prefix('[') {
        let Some(end) = rest.find(']') else {
            return Err(i18n::invalid_addr_v6(addr_str));
        };
        let after = &rest[end + 1..];
        if !after.starts_with(':') {
            return Err(i18n::invalid_addr(addr_str));
        }
        (&rest[..end], &after[1..])
    } else {
        let Some((ip, port)) = addr_str.rsplit_once(':') else {
            return Err(i18n::invalid_addr_format(addr_str));
        };
        (ip, port)
    };

    let ip = match ip_str {
        "*" | "" => None,
        _ => Some(
            ip_str
                .parse::<IpAddr>()
                .map_err(|_| i18n::invalid_ip(ip_str))?,
        ),
    };
    let port = match port_str {
        "*" => None,
        _ => Some(
            port_str
                .parse::<u16>()
                .map_err(|_| i18n::invalid_port(port_str))?,
        ),
    };
    Ok((Target { ip, port }, proto))
}

// / 后为空（如 *:8080/）与省略协议均视为 *
fn parse_proto(proto_str: Option<&str>) -> Result<Proto, String> {
    match proto_str {
        Some(s) if !s.is_empty() => s.parse::<Proto>(),
        _ => Ok(Proto::Any),
    }
}

// 通配 IP 显示为 0.0.0.0；指定协议时带 /TCP、/UDP 后缀
pub fn format_target(target: &Target, proto: Proto) -> String {
    let ip = match target.ip {
        Some(ip) if ip.is_ipv6() => format!("[{ip}]"),
        Some(ip) => ip.to_string(),
        None => "0.0.0.0".to_string(),
    };
    let port = match target.port {
        Some(p) => p.to_string(),
        None => "*".to_string(),
    };
    let mut s = format!("{ip}:{port}");
    match proto {
        Proto::TCP => s.push_str("/TCP"),
        Proto::UDP => s.push_str("/UDP"),
        Proto::Any => {}
    }
    s
}
